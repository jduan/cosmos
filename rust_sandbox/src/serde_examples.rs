use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;
use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub classname: String,
    pub name: String,
    pub time: f64,
    pub failure: Option<Vec<String>>,
    pub error: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct TestSuite {
    // This should be the path to the test file.
    pub name: String,
    pub errors: u32,
    pub failures: u32,
    // Some test runners don't produce this field. We don't really use this field now.
    pub skipped: Option<u32>,
    pub tests: u32,
    #[serde(rename = "testcase", default)]
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
pub struct TestSuites {
    #[serde(rename = "time", default)]
    pub duration_in_seconds: f64,
    pub name: String,
    pub failures: u32,
    pub tests: u32,
    #[serde(rename = "testsuite", default)]
    pub test_suites: Vec<TestSuite>,
}

pub fn parse_xml_file(junit_xml_file: &std::path::PathBuf) -> TestSuites {
    let file = File::open(junit_xml_file).unwrap_or_else(|err| {
        panic!(
            "Failed to read junit xml file {}. Error: {:?}",
            junit_xml_file.display(),
            err
        )
    });
    let buf_reader = BufReader::new(file);
    from_reader(buf_reader).unwrap_or_else(|err| {
        panic!(
            "Failed to parse junit xml file: {:?}. Error: {:?}",
            junit_xml_file, err
        )
    })
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use crate::serde_examples::parse_xml_file;

    #[test]
    fn test_xml_parsing() {
        let file = build_xml_file_bad().unwrap();
        let xml_file = file.path().to_path_buf();
        let test_suites = parse_xml_file(&xml_file);
        println!("test_suites: {:?}", test_suites);
    }

    fn build_xml_file_bad() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        let text = r##"
<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="jest tests" tests="4" failures="1" time="0.8">
  <testsuite name="__tests__/util.spec.js" errors="1" failures="1" skipped="0" timestamp="2019-11-22T21:19:15" time="0.391" tests="4">
    <testcase classname="Filter function-it should filter by a search term (link)" name="Filter function-it should filter by a search term (link)" time="0.002">
    </testcase>
    <testcase classname="Filter function-this test is meant to be flaky" name="Filter function-this test is meant to be flaky" time="0">
    </testcase>
    <testcase classname="Filter function-this test is meant to be flaky" name="Filter function-this test is meant to be flaky" time="0.002">
      <failure>Error: expect(received).toBe(expected) // Object.is equality

Expected: true
Received: false
    at Object.toBe (/Users/jingjing_duan/repos2/dummy_jest/__tests__/util.spec.js:22:33)
    at Object.asyncJestTest (/usr/local/lib/node_modules/jest/node_modules/jest-jasmine2/build/jasmineAsyncInstall.js:102:37)
    at /usr/local/lib/node_modules/jest/node_modules/jest-jasmine2/build/queueRunner.js:43:12
    at new Promise (&lt;anonymous&gt;)
    at mapper (/usr/local/lib/node_modules/jest/node_modules/jest-jasmine2/build/queueRunner.js:26:19)
    at /usr/local/lib/node_modules/jest/node_modules/jest-jasmine2/build/queueRunner.js:73:41
    at processTicksAndRejections (internal/process/task_queues.js:85:5)</failure>

      <failure> This just shows we allow multiple failure tags </failure>
    </testcase>
    <testcase classname="BotDetectionServiceTests" name="test_generateTokenAIRURLRequest_failsWhenProviderFails" time="10.839642">
      <error type="Error"></error>
      <error type="Error">Another error</error>
      <system-out>Child process terminated with signal 4: Illegal instruction
        Test crashed while running.</system-out>
    </testcase>
  </testsuite>
</testsuites>
        "##;
        writeln!(file, "{}", text)?;

        Ok(file)
    }
}
