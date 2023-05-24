// notes regarding c111 implementation of astm
//
// header
    // message_control_id 3rd field
    // access_password 4th field
    // 5th field is a CM type, i.e. component delimited
    // 6 through 9 not used
    // 11th field is a CM type, i.e. component delimited
    // 12th field is ST type
    // 13th field is NM type
    // 14th field is TS type

// legend: ST - string, NM - numerical, TS - timestamp, TM - time, DT - date,
// TX - optional text, CM - combination field

// termination
// sequence number is 1 by default
// termination code is limited to N and E

// patient
    // sequence_number is always 1
    // 3rd field is not used (practice patient id)
    // 4th field ignored when received by c111, when sent field has
    // un-extended sampleID part
    // 5 through 35 fields are ignored

// test_order
    // sequence_number represents the sequence at current layer, where layer
    // is reset to 1 for each new patient information record above, it is numbered
    // 1, 2, etc for each occurence of this record
    // 3rd field


// messages to be received by host from analyzer
// test_order_inquiry  -> TSREQ^REAL
// patient_control_sample_order_list ->
// patient_control_sample_analytical_data ->
// photometric_raw_data ->
// photometric_calibration_data ->
// ISE_calibration_data ->
// reagent_disk_loading ->
// database_factory_reset_response ->

// messages to be sent by host to analyzer
// analytical_data_transmission_request_sample
// analytical_data_transmission_request_cal
// test_orders
// reagent_tray_loading_inquiry
// database_factory_reset_request -> manufacturer use only
