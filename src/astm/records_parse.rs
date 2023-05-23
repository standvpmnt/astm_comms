
struct Header {
    raw_string: &string,
}

pub struct Delimiters {
    pub field_delimiter: &char, //2nd item
    pub repeat_delimiter: &char, //3rd item
    pub component_delimiter: &char, //4th item
    pub escapre_delimiter: &char //5th item
}

impl Header {
    fn delimiters(&self) -> Delimiters {
        todo!() //
    }

    fn message_control_id(&self) -> Option<&str> {
        todo!() // third field
    }

    fn access_password(&self) -> Option<&str> {
        todo!() // fourth field
    }

    fn sender_id(&self) -> Option<&str> {
        todo!() // fifth field
    }

    fn sender_street_address(&self) -> Option<&str> {
        todo!() //sixth field
    }

    fn reserved_field(&self) -> Option<bool> {
        None //seventh field
    }

    fn sender_telephone(&self) -> Option<&str> {
        todo!() //eighth field
    }

    fn sender_characteristics(&self) -> Option<&str> {
        todo!() //ninth field
    }

    fn receiver_id(&self) -> Option<&str> {
        todo!() //tenth field
    }

    // comment or special instructions eleventh field

    // processing id, twelvth field
    // P -> production
    // T -> training
    // D -> debugging
    // Q -> Quality control

    // version_number thirteenth field

    // date_time of message generation fourteenth field

    //



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



}
