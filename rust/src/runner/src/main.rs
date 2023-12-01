use std::env;

use anyhow::{bail, Result};

mod progress;
mod runner;

fn main() -> Result<()> {
    runner::init();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => progress::print_progress(),
        2 | 3 => {
            let year_str = &args[1];
            let year = year_str.parse::<u32>().unwrap();

            match args.len() {
                2 => match year {
                    2015 => runner::run_year::<y2015::Y2015, 2015>(),
                    2016 => runner::run_year::<y2016::Y2016, 2016>(),
                    2017 => runner::run_year::<y2017::Y2017, 2017>(),
                    2018 => runner::run_year::<y2018::Y2018, 2018>(),
                    2019 => runner::run_year::<y2019::Y2019, 2019>(),
                    2020 => runner::run_year::<y2020::Y2020, 2020>(),
                    2021 => runner::run_year::<y2021::Y2021, 2021>(),
                    2022 => runner::run_year::<y2022::Y2022, 2022>(),
                    _ => panic!("invalid year"),
                },
                3 => {
                    let day_str = &args[2];
                    let day_n = day_str.parse::<u32>()?;

                    match year {
                        2015 => runner::run_year_solution::<y2015::Y2015, 2015>(day_n),
                        2016 => runner::run_year_solution::<y2016::Y2016, 2016>(day_n),
                        2017 => runner::run_year_solution::<y2017::Y2017, 2017>(day_n),
                        2018 => runner::run_year_solution::<y2018::Y2018, 2018>(day_n),
                        2019 => runner::run_year_solution::<y2019::Y2019, 2019>(day_n),
                        2020 => runner::run_year_solution::<y2020::Y2020, 2020>(day_n),
                        2021 => runner::run_year_solution::<y2021::Y2021, 2021>(day_n),
                        2022 => runner::run_year_solution::<y2022::Y2022, 2022>(day_n),
                        _ => panic!("invalid year"),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => bail!("usage: [year] [day?]"),
    }

    Ok(())
}
