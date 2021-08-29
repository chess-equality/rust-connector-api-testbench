extern crate rust_connector_api;

fn main() {
    println!("Hello from test bench!");
}

// Use "develop" branch of rust_connector_api
#[cfg(test)]
mod tests {

    use crate::rust_connector_api::valid_date_time::{
        PeriodDate, PeriodTime, VDTOffset, ValidDateTime, ValidDateTimeBuilder,
    };
    use crate::rust_connector_api::MeteomaticsConnector;
    use chrono::{Duration, Local, Utc};
    use rust_connector_api::connector_response::CSVBody;
    use rust_connector_api::locations::{Coordinates, Locations};
    use rust_connector_api::optionals::{Opt, OptSet, Optionals};
    use rust_connector_api::parameters::{PSet, Parameters, P};
    use std::iter::FromIterator;

    #[tokio::test]
    async fn call_query_time_series_with_options() {
        println!("##### call_query_time_series_with_options:");

        // Create API connector
        let meteomatics_connector = MeteomaticsConnector::new(
            "python-community".to_string(),
            "Umivipawe179".to_string(),
            10,
        );

        // Create ValidDateTime
        let now = Local::now();
        let yesterday = VDTOffset::Local(now.clone() - Duration::days(1));
        let now = VDTOffset::Local(now);
        let local_vdt: ValidDateTime = ValidDateTimeBuilder::default()
            .start_date_time(yesterday)
            .end_date_time(now)
            .build()
            .unwrap();

        // Create Parameters
        let p_values: PSet<'_> = PSet::from_iter([
            P {
                k: "t_2m",
                v: Some("C"),
            },
            P {
                k: "precip_1h",
                v: Some("mm"),
            },
        ]);
        let parameters: Parameters = Parameters { p_values };

        // Create Locations
        let coordinates = Coordinates::from(["47.419708", "9.358478"]);
        let locations: Locations = Locations { coordinates };

        // Create Optionals
        let opt_values: OptSet<'_> = OptSet::from_iter([
            Opt {
                k: "source",
                v: "mix",
            },
            Opt {
                k: "calibrated",
                v: "true",
            },
        ]);
        let optionals: Optionals = Optionals { opt_values };

        // Call endpoint
        let response = meteomatics_connector
            .query_time_series(local_vdt, parameters, locations, Option::from(optionals))
            .await
            .unwrap();

        let csv_body = response.body;
        println!(">>>>>>>>>> CSV body:\n{}", csv_body);

        print!("\n>>>>>>>>>> CSV headers:\n");
        println!("{}", csv_body.csv_headers.to_vec().join(","));

        print!("\n>>>>>>>>>> CSV records:\n");
        for csv_record in csv_body.csv_records {
            println!("{}", csv_record.to_vec().join(","));
        }

        assert_eq!(response.http_status, "200 OK");

        assert_ne!(
            response.body,
            CSVBody {
                csv_headers: vec![],
                csv_records: vec![]
            }
        );
    }

    #[tokio::test]
    async fn valid_date_time_with_optional_params() {
        println!("##### valid_date_time_with_optional_params (UTC):");

        let start_date_time = Utc::now();
        let period_date = PeriodDate::Days(1);
        let end_date_time = start_date_time + Duration::days(1);
        let start_vdt_offset = VDTOffset::Utc(start_date_time);
        let end_vdt_offset = VDTOffset::Utc(end_date_time);
        let time_step = PeriodTime::Hours(1);
        let time_list = vec![start_vdt_offset, end_vdt_offset];

        let utc_vdt: ValidDateTime = ValidDateTimeBuilder::default()
            .start_date_time(start_vdt_offset)
            .period_date(period_date)
            .end_date_time(end_vdt_offset)
            .time_step(time_step)
            .time_list(time_list)
            .build()
            .unwrap();

        println!(
            ">>>>>>>>>> utc_vdt.start_date_time: {:?}",
            utc_vdt.start_date_time
        );
        println!(
            ">>>>>>>>>> utc_vdt.period_date: {}",
            utc_vdt.period_date.unwrap()
        );
        println!(
            ">>>>>>>>>> utc_vdt.end_date_time: {:?}",
            utc_vdt.end_date_time.unwrap()
        );
        println!(
            ">>>>>>>>>> utc_vdt.time_step: {}",
            utc_vdt.time_step.unwrap()
        );
        println!(">>>>>>>>>> utc_vdt.time_list: {:?}", utc_vdt.time_list);

        assert_eq!(
            utc_vdt,
            ValidDateTime {
                start_date_time: start_vdt_offset,
                period_date: Some(period_date),
                end_date_time: Some(end_vdt_offset),
                time_step: Some(time_step),
                time_list: Some(vec![start_vdt_offset, end_vdt_offset])
            }
        );

        assert_eq!(utc_vdt.period_date.unwrap(), PeriodDate::Days(1));
        assert_eq!(utc_vdt.time_step.unwrap(), PeriodTime::Hours(1));

        assert_eq!(utc_vdt.period_date.unwrap().to_string(), "P1D");
        assert_eq!(utc_vdt.time_step.unwrap().to_string(), "PT1H")
    }

    #[tokio::test]
    async fn parameters_with_some_values() {
        println!("##### parameters_with_some_values:");

        let mut p_values: PSet<'_> = PSet::new();
        let p1 = P {
            k: "t_2m",
            v: Some("C"),
        };
        let p2 = P {
            k: "precip_1h",
            v: Some("mm"),
        };
        p_values.push(p1);
        p_values.push(p2);

        let params: Parameters = Parameters { p_values };

        println!(">>>>>>>>>> params: {}", params);

        assert_eq!(params.to_string(), "t_2m:C,precip_1h:mm");

        assert_ne!(
            params.p_values,
            PSet::from_iter([P {
                k: "t_2m",
                v: Some("C")
            }])
        );
    }
}
