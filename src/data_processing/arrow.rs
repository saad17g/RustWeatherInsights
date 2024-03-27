use arrow::array::Float64Array;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

pub fn process_weather_data(
    data: Vec<(f64, f64)>,
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Err("No data points provided".into());
    }
    let temperatures = data.iter().map(|(temp, _)| *temp).collect::<Vec<_>>();
    let humiditites = data
        .iter()
        .map(|(_, humidity)| *humidity)
        .collect::<Vec<_>>();

    let temp_arr = Float64Array::from(temperatures);
    let hum_arr = Float64Array::from(humiditites);

    let schema = Schema::new(vec![
        Field::new("temperature", DataType::Float64, false),
        Field::new("humidity", DataType::Float64, false),
    ]);

    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(temp_arr), Arc::new(hum_arr)],
    )?;

    let temp_sum = batch
        .column(0)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .values()
        .iter()
        .sum::<f64>();

    let humidity_sum = batch
        .column(1)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .values()
        .iter()
        .sum::<f64>();

    let count = batch.num_rows() as f64;

    let avg_temp = temp_sum / count;
    let avg_humidity = humidity_sum / count;

    Ok((avg_temp, avg_humidity))
}
