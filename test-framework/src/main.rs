use crate::{ExpectedPassFail, TestCase, TestSuite};

fn main() {
    let suite = TestSuite::new(vec![TestCase::new(
        "/Users/ed/Projects/oatmilk/sample-files/ifq.oat",
        ExpectedPassFail::CompileFail,
    )]);

    suite.run_all()
}
