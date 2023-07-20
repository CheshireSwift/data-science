use polars::{lazy::dsl::StrptimeOptions, prelude::*};

fn main() -> Result<(), PolarsError> {
    let raw = LazyCsvReader::new("./data/scottish_epc_ratings.csv")
        .finish()?
        .select([
            col("*").exclude(["date"]),
            col("date").str().to_date(StrptimeOptions::default()),
        ]);

    let up_to_date = raw
        .groupby_stable(["uprn"])
        .agg([col("*").last()])
        .filter(col("uprn").is_not_null())
        .select([col("date"), col("uprn"), col("epc_rating")]);
    // println!("{:?}", up_to_date.collect()?);

    let mut file = std::fs::File::create("latest_scottish_epc_ratings.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut up_to_date.collect()?)?;

    Ok(())
}
