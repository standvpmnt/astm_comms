use bytes::Bytes;

#[derive(Debug, PartialEq)]
pub enum Record {
    // indicators are case insensitive
    Header(Bytes),                  //H
    Patient(Bytes),                 //P
    TestOrder(Bytes),               //O
    ResultR(Bytes),                  //R
    Comment(Bytes),                 //C
    RequestInformation(Bytes),      //Q
    Scientific(Bytes),              //S
    MessageTerminator(Bytes),       // L
    ManufacturerInformation(Bytes), //M
}

#[derive(thiserror::Error, Debug)]
pub enum RecordError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    MalformedRecord(String),
}

impl Record {
    pub fn parse_from_buf(buf_slice: &[u8]) -> Result<Record, RecordError> {
        if buf_slice.len() < 3 {
            return Err(RecordError::MalformedRecord(format!(
                "provided buffer is not an ASTM record, {:?}",
                buf_slice
            )));
        }
        // println!("parsing buf slice: {:?}", buf_slice);
        match buf_slice[2] {
            b'h' | b'H' => Ok(Record::Header(Bytes::copy_from_slice(buf_slice))),
            b'p' | b'P' => Ok(Record::Patient(Bytes::copy_from_slice(buf_slice))),
            b'o' | b'O' => Ok(Record::TestOrder(Bytes::copy_from_slice(buf_slice))),
            b'r' | b'R' => Ok(Record::ResultR(Bytes::copy_from_slice(buf_slice))),
            b'c' | b'C' => Ok(Record::Comment(Bytes::copy_from_slice(buf_slice))),
            b'q' | b'Q' => Ok(Record::RequestInformation(Bytes::copy_from_slice(
                buf_slice,
            ))),
            b's' | b'S' => Ok(Record::Scientific(Bytes::copy_from_slice(buf_slice))),
            b'l' | b'L' => Ok(Record::MessageTerminator(Bytes::copy_from_slice(buf_slice))),
            b'm' | b'M' => Ok(Record::ManufacturerInformation(Bytes::copy_from_slice(
                buf_slice,
            ))),
            _ => Err(RecordError::MalformedRecord(format!(
                "record has some other item,{}",
                buf_slice[2]
            ))),
        }
    }
}

#[derive(Debug)]
pub struct Header {
    raw_data: Bytes,
    pub field_delim: u8,
    pub repeat_delim: u8,
    pub component_delim: u8,
    pub escape_delim: u8,
}

impl Header {
    // TODO! modify this to use Record instead
    pub fn new(record: Record) -> Header {
        if let Record::Header(bytes) = record {
            if bytes.len() < 6 {
                panic!("unhandled case of short buffer for header {:?}", bytes);
            }
            if bytes[2] == b'H' || bytes[2] == b'h' {
                return Header {
                    field_delim: bytes.get(3).map(|x| x.clone()).unwrap_or(b'|'),
                    repeat_delim: bytes.get(4).map(|x| x.clone()).unwrap_or(b'\\'),
                    component_delim: bytes.get(5).map(|x| x.clone()).unwrap_or(b'^'),
                    escape_delim: bytes.get(6).map(|x| x.clone()).unwrap_or(b'&'),
                    raw_data: bytes,
                }
            } else {
                println!("{:?}", *bytes.get(2).unwrap());
                println!("{:?}, {:?}", b'h', b'H');
                panic!("attempting to parse invalid record,\n {:?}", bytes);
            }
        } else {
            panic!("attempted to create Header from invalid record");
        }
    }

    pub fn message_control_id(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(2)
    }

    pub fn access_password(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(3)
    }

    pub fn sender_id(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(4)
    }

    pub fn sender_street_address(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(5)
    }

    pub fn reserved_field(&self) -> Option<bool> {
        None //seventh field
    }

    pub fn sender_telephone(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(7)
    }

    pub fn sender_characteristics(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(8).filter(|x| x != &[])
    }

    pub fn receiver_id(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(9)
    }

    pub fn special_instructions(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(10)
    }

    pub fn processing_id(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(11)
    // P -> production
    // T -> training
    // D -> debugging
    // Q -> Quality control
    }

    pub fn version_number(&self) -> Option<&[u8]> {
        // this processing is being done because the item may be `[]`
        // which should not be interpreted as ""
        self.raw_data.split(|x| x == &self.field_delim).nth(12).filter(|x| x != &[])
    }

    pub fn sent_at(&self) -> Option<&[u8]> {
        self.raw_data.split(|x| x == &self.field_delim).nth(13)
    }

}

struct Patient {}

impl Patient {
    // sequence_number second field 5.6.7

    // doctors_id third field

    // lab_id fourth field

    // other_id_number_optional fifth field

    // name sixth field, (last name, first name, middle name or initial, suffix, title
    //                            separated by component delimieter 5.6.6)

    // mothers_maiden_name (mother's maiden surname) seventh field

    // birthdate (5.6.2) eighth field

    // gender M, F, U ninth field

    // ethnic_origin tenth field, multiple answers are permitted
    // W - white
    // H - hispanic
    // B - black
    // O - Asian/Pacific Islander
    // NA - North American/Alaskan Native

    // address eleventh field (5.6.5)

    // reserved_field twelvth field

    // telephone_number thirteenth field (5.6.3)

    // physician_id (multiple physicians if applicable need to be separated by repeat
    //               delimiter) can be code or names fourteenth field

    // special_field_one fifteenth field

    // special_field_two sixteenth field

    // height (default unit of cms) seventeenth field

    // weight (default unit of kgs) other units refer to 5.6.4 eighteenth field

    // suspected_diagnosis (ICD-9 code or free text, multiple separated by repeat)
    // nineteenth field

    // active_medications (generic names should be used) twenteth field

    // diet free text field, fasting or not indicated here (16-hour for Tg)
    // twenty first field

    // practice_field_1 (optional field) twenty-secondth field

    // practice_field_2 (optional field) twenty-thirdth field

    // admission_discharge_date (discharge date should follow admission date, separated by
    //                           repear delimiter) twenty fourth field

    // admission_status (can be extended by generally will be the following options)
    // OP - Outpatient
    // PA - Preadmit
    // IP - Inpatient
    // ER - Emergency room
    // twenty-fifth field

    // Location (clinic location of the patient, mutually agreed upon) twenty sixth field

    // nature_alternative_diagnostic_code_and_classifiers identifies class of code transmitted in next field

    // alternative_diagnostic_code_and_classifiers DRG codes can be passed here, repeat delimiters for multiple codes
    // individual codes can be followed with optional test descriptors and must be separated by component delimieter

    // religion (mutually agreed by sender and receiver for encoding, sample below) twenty ninth field
    // P - Protestant
    // C - Catholic
    // M - Mormon
    // J - Jewish
    // L - Lutheran
    // H - Hindu

    // marital_status thirtyth field
    // M - Married
    // S - Single
    // D - Divorced
    // W - Widowed
    // A - Separated

    // isolation_status thirty-first field, suggested codes for common precaution,
    // multiple precautions separated by repeat delimiters
    // ARP - Antibiotic resistance precautions
    // BP - blood and needle precautions
    // ENP - enteric precautions
    // NP - precuations for neutropenic patient
    // PWP - precautions for pregnant women
    // RI - respiratory isolation
    // SE - secretion/excretion precautions
    // SI - strict isolation
    // WSP - wound and skin precautions

    // Language patient's primary language thirty secondth field

    // hospital_service hospital service currently assigned to patient, code and text
    //     may be sent separated by component delimiter thirty third field

    // hospital_institution hospital currently assigned to the patient code and text
    //     may be ent when separated by component delimiter

    // dosage category subcomponents can be used to define dosage subgroups,
    // A - Adult
    // P1 - Pediatric (one to six months)
    // P2 - Pediatric (six months to three years)
}

struct TestOrder {} // O

impl TestOrder {
    // sequence_number second field

    // specimen_id, if multiple components of specimen separate them with
    // component delimiter third field

    // instrument_specimen_id text field, identifier assigned by the instrument fourth field

    // universal_test_id this is the universal test id refer to 5.6.1 fifth field

    // priority test priority, if multiple they must be separated by repeat Delimiters
    // S - static
    // A - as soon as possible
    // R - routine
    // C - callback
    // P - preoperative
    // sixth field

    // requested_date_and_time seventh field

    // collection_date_and_time eighth field

    // collection_end_time ninth field

    // collection_volume default unit is milliliters separate from numeric value by
    // component delimiter for units convention refer to 5.6.4 tenth field

    // collector_id person who collected the specimen eleventh field

    // action_code codes for handling/action to be taken with specimen
    // C - cancel request for the battery or tests named
    // A - add reqeusted tests to the existing specimen with the patient and specimen
    // identifiers and date time given in this record
    // N - new request accompanying a new specimen
    // P - pending specimen
    // L - reserved
    // X - speciment or test already in progress
    // Q - treat specimen as Q/C test specimen

    // danger_code indicate any special hazard with the specimen eg. hepatitis patient, suspected anthrax etc.
    // thirteenth field

    // clinical_information any information related to test fourteenth field

    // specimen_received_at sample receieved in lab at this time fifteenth field

    // specimen_descriptor this has 2 parts separated by component delimiter
    //     specimen_type eg. blood, urine, serum, hair, wound, biopsy, sputum etc. this is not
    //     available in c111
    //     specimen_source this is specimen source body type

    // ordering_physician name of ordering physician seventeenth field

    // physician_contact_number see 5.6.3 for format, eighteenth field

    // user_field_1 nineteenth
    // user_field_2 twenteth
    // laboratory_field_1 twenty-first
    // laboratory_field_2 twenty-second

    // results_reported_or_modified_at twenty-thirdth field

    // instrument_charge billing charge of instrument for tests performed 24th field

    // instrument_section_id section of instrument where test was performed, eg. position
    //     in line 25th field

    // report types 26th field with below codes
    // O - order record; user asking that analysis be performed
    // C - correction of previously transmitted results
    // P - preliminary results
    // F - final results
    // X - order cannot be done, order cancelled
    // I - in instrument pending
    // Y - no order on record for this test (in response to query)
    // Z - no record of this patient (in response to query)
    // Q - response to query (this record is a response to a request-information query)

    // reserved_field 27th field

    // location_of_specimen_collection 28th field

    // nosocomical_infection_flag 29th field for epidemiological reporting purposes,
    // i.e. the organism identified is from hospital acquired infection

    // specimen service define specific service responsible for such collection
    // 30th field

    // specimen_institution if specimen is collected at institution other than
    // patient institution
}

struct ResultR {} // R

impl ResultR {
    // sequence_number 2nd field

    // universal_test_id 3rd field

    // data/measurement value 4th field, avoid components in this field when possible

    // units 5th field, ISO standard abbreviations in accordance with ISO 2955 when
    // available, units can be in upper case or lower case

    // reference_ranges lower limit to upper limit, multiple ranges separated by
    // repeat delimiters, range can contain text description and it is separated by
    // component delimiter

    // abnormal_flag indicates normalcy status of the result 7th field
    // L - below low normal
    // H - above high normal
    // LL - below panic normal
    // HH - above panic high
    // < - below absolute low, that is off low scale on instrument
    // > - above absolute high, off high scale on instrument
    // N - normal
    // A - abnormal
    // U - significant change up
    // D - significant change down
    // B - better, use when direction not relevant or not defined
    // W - worse, use when direction not relevant or not defined

    // nature_of_abnormality_testing kind of normal testing performed, all codes are
    // included which are applicable eg. ASR can be a realistic entry in this field
    // 8th field
    // A - age based population was tested
    // S - sex based population was tested
    // R - race based population was tested
    // N - generic normal range was applied to all patient specimens

    // status 9th field following codes
    // C - correction of previously transmitted results
    // P - preliminary results
    // F - final results
    // X - order cannot be done
    // I - in instrument, results pending
    // S - partial results
    // M - this result is an MIC level
    // R - this result was previously transmitted
    // N - this result contains necessary information to run a new order
    // Q - this result is a response to an outstanding query
    // V - operator verified/approved result
    // W - warning: validity is questionable

    // change_in_normative_value_date 10th field

    // operator_identification 11th field, component delimited to indicate first who
    // conducted the test, and second who verified the test

    // test_started_at date and time when the test started in the instrument 12th field

    // test_completed_at date and time when the test completed in the instrument 13th field

    // instument_identification 14th field identifies the instument or section of instrument
    // which performed this test
}

struct Comment {} // C

impl Comment {
    // sequence_number refer to section 5.6.7 2nd field

    // source comment origination point with below codes 3rd field
    // P - practice
    // L - information system
    // I - clinical instrument system

    // text comment text code etc can be sent by using the component delimiter 4th field_delimiter
}

struct RequestInformation {} // Q

impl RequestInformation {
    // sequence_number refer to section 5.6.7 2nd field

    // starting_range_id 3rd field, can contain 3 or more components,
    // first component => patient_id number of information system
    // second component => specimen_id number of information system
    // other components => manufacturer defined and position dependent
    // a list of sample ids can be request by tusing the repeat delimiter to separate ids

    // ending_range_id 4th field if single record then this is left blank

    // universal_test_id 5th field this contains the test id or the term `ALL` indicating
    // a request for data pertaining to all tests associated with range specified above

    // nature_of_time_limits 6th field, encoded as below and indicate what the time limits
    // are for
    // S - specimen collection date
    // R - result test date
    // None - assume result test date

    // beginning_request_results_date_and_time 7th field, if empty this should be as
    // old as possible, can contain a list of datetime separated by repeat delimiter

    // ending_request_results_date_and_time 8th field if not null specifies the ending
    // or latest date for which results are being requested

    // requesting_physician_name 9th field, identifies physician identity associated
    // with a request as per 5.6.6

    // requesting_physician_telephone 10th field

    // user_field_1 11th field
    // user_field_2 12th field

    // request_information_status_codes 13th field
    // C - correction of previously transmitted results
    // P - preliminary results
    // F - final results
    // X - results cannot be done, request cancelled
    // I - request results pending
    // S - request partial/unfinalized results
    // M - results in MIC level
    // R - this result was previously transmitted
    // A - abort/cancel last request criteria (allows a new request to follow)
    // N - requesting new or edited results only
    // O - requesting test orders and demographics only (no results)
    // D - requesting demographics only (eg. patient record)
}

struct MessageTerminator {} // L

impl MessageTerminator {
    // sequence_number 2nd field

    // termination_code 3rd field with below codes
    // Nil, N - normal termination
    // T - sender aborted
    // R - receiver requested abort
    // E - unknown system error
    // Q - error in last request for information // terminate a request and allow processing of a
    //                                           // new request record
    // I - no information available from last query // terminate a request and allow processing of a
    //                                              // new request record
    // F - last request for information processed
}

struct Scientific {} // S

impl Scientific {
    // sequence_number 2nd field

    // analytical method 3rd field, text field conforms to Appendix I of Elevitch and Boroviczeny

    // instrumentation 4th field, id composed of manufacturer and instrument codes
    // connected by dash, codes will also conform to Appendix I of Elevitch and Boroviczeny

    // reagents 5th field, text field which is list of constituent reagent codes separated by
    // subfield ID

    // units_of_measure 6th field

    // quality_control 7th field specs pending

    // specimen_descriptor 8th field

    // reserved_field 9th field

    // container 10th field, specs pending

    // specimen_id 11th field, unique specimen identified sent by originator and returned by
    // receiving instrument

    // analyte 12th field specs pending

    // result 13th field represents the determined value of the analyte, numeric field

    // result_units 14th field

    // collection_date_time 15th field

    // result_date_time 16th field

    // analytical_preprocessing_steps 17th field text field description of any steps

    // patient_diagnosis 18th field ICD-9 CM codes

    // patient_birthdate 19th field

    // patient_gender 20th field

    // patient_race 21st field
}

struct ManufacturerInformation {} // M

impl ManufacturerInformation {
    // sequence_number 2nd field

    // manufacturer specific, details keep changing for vendors
}

#[cfg(test)]
mod tests {
    use super::Record;
    use claims::*;

    #[test]
    fn can_parse_records() {
        // b"1H|\\^&|||c111^Roche^c111^4.2.2.1730^1^13085|||||host|PCUPL^BATCH|P|1|20230515160340\r";
        let head_record: &[u8] = &[
            2, 49, 72, 124, 92, 94, 38, 124, 124, 124, 99, 49, 49, 49, 94, 82, 111, 99, 104, 101,
            94, 99, 49, 49, 49, 94, 52, 46, 50, 46, 50, 46, 49, 55, 51, 48, 94, 49, 94, 49, 51, 48,
            56, 53, 124, 124, 124, 124, 124, 104, 111, 115, 116, 124, 82, 83, 85, 80, 76, 94, 66,
            65, 84, 67, 72, 124, 80, 124, 49, 124, 50, 48, 50, 51, 48, 53, 50, 53, 49, 54, 52, 57,
            51, 51, 13, 23, 70, 68, 13, 10,
        ];
        let patient_record: &[u8] = &[2, 50, 80, 124, 49, 124, 124, 13, 23, 52, 66, 13, 10];
        let order_record: &[u8] = &[
            2, 51, 79, 124, 49, 124, 80, 67, 67, 67, 49, 94, 53, 50, 53, 48, 50, 55, 48, 48, 94,
            50, 48, 50, 51, 49, 50, 51, 49, 124, 49, 51, 57, 49, 124, 94, 94, 94, 55, 49, 50, 124,
            124, 124, 124, 124, 124, 124, 81, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124,
            124, 50, 48, 50, 51, 48, 53, 50, 53, 49, 54, 52, 57, 51, 51, 124, 124, 124, 70, 13, 23,
            48, 50, 13, 10,
        ];
        let result_record: &[u8] = &[
            2, 52, 82, 124, 49, 124, 94, 94, 94, 55, 49, 50, 124, 48, 46, 57, 124, 109, 103, 47,
            100, 76, 124, 49, 46, 48, 92, 48, 46, 57, 92, 49, 46, 48, 124, 78, 124, 124, 82, 124,
            124, 36, 83, 89, 83, 36, 124, 124, 50, 48, 50, 51, 48, 52, 50, 56, 49, 56, 52, 49, 49, 54, 13, 23, 67, 55, 13, 10, ];
        let comment_record: &[u8] = &[
            2, 53, 67, 124, 49, 124, 73, 124, 124, 73, 13, 23, 52, 70, 13, 10,
        ];
        let termination_record: &[u8] = &[2, 54, 76, 124, 49, 124, 78, 13, 3, 48, 57, 13, 10];

        let output = Record::parse_from_buf(head_record).expect("failed to parse string");
        match output {
            Record::Header(k) => assert!(k.len() > 1),
            _ => {
                println!("Failed to parse header record \n {:?}", head_record);
                assert_err!(Ok(5));
            }
        }

        let output = Record::parse_from_buf(patient_record).expect("failed to parse string");
        match output {
            Record::Patient(k) => assert!(k.len() > 1),
            _ => {
                println!("Failed to parse patient record \n {:?}", patient_record);
                assert_err!(Ok(5));
            }
        }

        let output = Record::parse_from_buf(order_record).expect("failed to parse string");
        match output {
            Record::TestOrder(k) => assert!(k.len() > 1),
            _ => {
                println!("Failed to parse order record \n {:?}", order_record);
                assert_err!(Ok(5));
            }
        }

        let output = Record::parse_from_buf(result_record).expect("failed to parse string");
        match output {
            Record::ResultR(k) => {
                assert!(k.len() > 1);
            },
            _ => {
                println!("Failed to parse result record \n {:?}", result_record);
                assert_err!(Ok(5));
            }
        }

        let output = Record::parse_from_buf(comment_record).expect("failed to parse string");
        match output {
            Record::Comment(k) => assert!(k.len() > 1),
            _ => {
                println!("Failed to parse comment record \n {:?}", comment_record);
                assert_err!(Ok(5));
            }
        }

        let output = Record::parse_from_buf(termination_record).expect("failed to parse string");
        match output {
            Record::MessageTerminator(k) => assert!(k.len() > 1),
            _ => {
                println!("Failed to parse result record \n {:?}", termination_record);
                assert_err!(Ok(5));
            }
        }
    }

    #[test]
    fn invalid_inputs_are_adequately_handles() {
        let input = b"14";
        let input1 = b"";
        let input2 = b"ajdf";

        assert_err!(Record::parse_from_buf(input));
        assert_err!(Record::parse_from_buf(input1));
        assert_err!(Record::parse_from_buf(input2));
    }

    // #[test]
    // fn can_get_frame_number_of_a_record() {
    //     let input = "1H|\\^&|||c111^Roche^c111^4.2.2.1730^1^13085|||||host|PCUPL^BATCH|P|1|20230515160340\r";
    //     let input1 = "2M|1|CR^BM^c111^1|712^BILT3|57884601|umol/L|BS^BILT3|712^SR^12547\\712^R1^1209|N^R|2|20230428183346|A^$SYS$||1.349997E-03^2.383310E-04|SD^^^59514300|70.7^0.0182^0.0186^0.0178^0^0\\0^0.00135^0.0016^0.0011^0^0\r";
    //     let input2 = "3L|1|N\r";

    //     assert_eq!(Record::parse(input.to_owned()).unwrap().frame_number(), 1);
    //     assert_eq!(Record::parse(input1.to_owned()).unwrap().frame_number(), 2);
    //     assert_eq!(Record::parse(input2.to_owned()).unwrap().frame_number(), 3);
    // }
}
