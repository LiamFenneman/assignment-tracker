use tracker_core::{
    assignment::Assignment,
    course::Course,
    mark::{grade, Grade, Mark, Percent},
};

#[test]
fn average_mark() {
    let mut c = Course::new(
        0,
        "CS 101".to_string(),
        "Introduction to Computer Science".to_string(),
    );
    c.add_assignment(
        Assignment::new(0, c.id(), "Assignment 1".to_string()) //
            .with_mark(Mark::percent(75).expect("valid percent")),
    );
    c.add_assignment(
        Assignment::new(1, c.id(), "Assignment 2".to_string()) //
            .with_mark(Mark::grade(Grade::B(Some(grade::Modifier::Plus)))),
    );
    c.add_assignment(
        Assignment::new(2, c.id(), "Assignment 3".to_string()) //
            .with_mark(Mark::out_of(15, 20).expect("valid mark")),
    );

    assert_eq!(
        c.average_mark(),
        Some(Percent::new(75).expect("valid percent"))
    );
}
