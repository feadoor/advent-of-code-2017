pub trait DepthFirstTree where Self: Sized {
    type Step: Sized;
    type Output;

    fn next_steps(&mut self) -> Vec<Self::Step>;
    fn apply_step(&mut self, node: &Self::Step);
    fn revert_step(&mut self, node: &Self::Step);
    fn output(&mut self) -> Option<Self::Output>;

    fn iter(&mut self) -> DepthFirstSearcher<Self> {
        DepthFirstSearcher::new(self)
    }
}

enum Step<T: DepthFirstTree> {
    Apply(T::Step),
    Revert(T::Step),
    StartSearch,
    EndSearch,
}

pub struct DepthFirstSearcher<'a, T: 'a + DepthFirstTree> {
    tree: &'a mut T,
    steps: Vec<Step<T>>,
}

impl<'a, T: 'a + DepthFirstTree> DepthFirstSearcher<'a, T> {
    fn new(tree: &'a mut T) -> DepthFirstSearcher<'a, T> {
        DepthFirstSearcher { tree: tree, steps: vec![Step::StartSearch] }
    }

    fn apply_step(&mut self, step: T::Step) {
        self.tree.apply_step(&step);
        self.steps.push(Step::Revert(step));
    }

    fn add_child_steps(&mut self) {
        self.steps.extend(self.tree.next_steps().into_iter().rev().map(|step| Step::Apply(step)));
    }
}

impl<'a, T: DepthFirstTree> Iterator for DepthFirstSearcher<'a, T> {
    type Item = T::Output;

    fn next(&mut self) -> Option<T::Output> {
        use self::Step::*;

        while let Some(step) = self.steps.pop() {
            match step {
                StartSearch => {
                    self.steps.push(EndSearch);
                    self.add_child_steps();
                },
                EndSearch => {
                    if let Some(value) = self.tree.output() {
                        return Some(value);
                    }
                },
                Apply(step) => {
                    self.apply_step(step);
                    self.add_child_steps();
                },
                Revert(step) => {
                    let output = self.tree.output();
                    self.tree.revert_step(&step);
                    if let Some(value) = output {
                        return Some(value);
                    }
                }
            }
        }

        None
    }
}