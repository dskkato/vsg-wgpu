use super::StimulusRenderer;

pub struct Scene {
    stimuli: Vec<Box<dyn StimulusRenderer>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            stimuli: Vec::new(),
        }
    }

    pub fn add_stimulus(&mut self, stimulus: Box<dyn StimulusRenderer>) {
        self.stimuli.push(stimulus);
    }
}

impl StimulusRenderer for Scene {
    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass,
    {
        for stimulus in &self.stimuli {
            stimulus.render(render_pass);
        }
    }
}
