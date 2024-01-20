use std::thread;
use std::time::Duration;
use windows::system::Diagnostics::Performance::{
    IPerformanceCounter, IPerformanceCounterSet, PerformanceCounter, PerformanceCounterSet,
};
use windows::system::Diagnostics::ProcessDiagnosticInfo;
use windows::Win32::System::SystemServices::HANDLE;
use windows::{core::*, Win32::System::Performance::*};
use windows::{Interface, IntoParam, Result, HRESULT};

fn main() -> Result<()> {
    let process_id = 1234; // Replace with the ID of your process

    let process_info = ProcessDiagnosticInfo::GetForProcessId(process_id)?;

    let counter_set = PerformanceCounterSet::new("{8c03567f-31d1-4ece-97b5-7d8d5e78c9f5}")?;
    let counter = PerformanceCounter::new(
        &counter_set,
        "% Processor Time",
        &format!("PID_{}", process_id),
    )?;

    loop {
        let value = counter.Get()?;

        println!(
            "Process {} ({}) CPU usage: {}%",
            process_info.ExecutableFileName()?,
            process_id,
            value
        );

        thread::sleep(Duration::from_secs(1));
    }
}

fn main2() {
    unsafe {
        let mut query = 0;
        PdhOpenQueryW(None, 0, &mut query);

        let mut counter = 0;
        let pid = 2219;
        let query_string = w!("\\Process(2219)\\% Processor Time");
        PdhAddCounterW(
            query,
            // w!("\\Processor(0)\\% Processor Time"),
            w!("\\Process(2219)\\% Processor Time"),
            0,
            &mut counter,
        );

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            PdhCollectQueryData(query);

            let mut value = std::mem::zeroed();
            if 0 == PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value) {
                println!("{:.2}", value.Anonymous.doubleValue);
            }
        }
    }
}
