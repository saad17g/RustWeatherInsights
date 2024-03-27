use crate::data_processing::arrow;

#[test]
fn test_process_weather_data() {
    // Test case 1: Empty input
    let data = vec![];
    let result = arrow::process_weather_data(data);
    assert!(result.is_err());

    // Test case 2: Single data point
    let data = vec![(25.5, 60.0)];
    let result = arrow::process_weather_data(data).unwrap();
    assert_eq!(result, (25.5, 60.0));

    // Test case 3: Multiple data points
    let data = vec![(25.5, 60.0), (27.3, 65.0), (24.8, 62.0)];
    let result = arrow::process_weather_data(data).unwrap();
    let expected_temp = (25.5 + 27.3 + 24.8) / 3.0;
    let expected_humidity = (60.0 + 65.0 + 62.0) / 3.0;
    assert!((result.0 - expected_temp).abs() < 0.001);
    assert!((result.1 - expected_humidity).abs() < 0.001);
}
