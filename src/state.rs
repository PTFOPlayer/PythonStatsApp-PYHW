use iced::widget::{text, Text};

use crate::{components::chart::Graph, error::AppError};

pub struct State {
    pub fails: Fails,
    pub gpu: GpuState,
    pub graphs_switch: bool,
    pub cpu_pwr_graph: Graph,
    pub cpu_temp_graph: Graph,
    pub cpu_usage_graph: Graph,
    pub cpu_avg_freq_graph: Graph,
}

impl Default for State {
    fn default() -> Self {
        Self {
            fails: Fails::default(),
            gpu: GpuState::None,
            graphs_switch: false,
            cpu_pwr_graph: Graph::new(50f32, "Cpu Power (W)"),
            cpu_temp_graph: Graph::new(100f32, "Cpu Temperature (°C)"),
            cpu_usage_graph: Graph::new(100f32, "Cpu Usage (%)"),
            cpu_avg_freq_graph: Graph::new(2500f32, "Cpu avarage frequency (MHz)"),
        }
    }
}

#[derive(Default)]
pub struct Fails {
    pub msr_fail: Option<AppError>,
    pub sys_fail: Option<AppError>,
}

pub enum GpuState {
    None,
    Radeon,
    Nvidia,
}

pub struct StaticElements<'a> {
    pub cpu_title: Text<'a>,
    pub cpu_cache: Vec<(Text<'a>, Text<'a>)>,
    pub cores_threads: (Text<'a>, Text<'a>),
}

impl<'a> Default for StaticElements<'a> {
    fn default() -> Self {
        Self {
            cpu_title: text("Unknown"),
            cpu_cache: vec![],
            cores_threads: (text(""), text("")),
        }
    }
}

pub struct AxisState {
    pub divider: Option<u16>,
    pub split_axis: iced_aw::split::Axis,
    pub axis_state: bool,
}

impl Default for AxisState {
    fn default() -> Self {
        Self {
            divider: Default::default(),
            split_axis: Default::default(),
            axis_state: Default::default(),
        }
    }
}

impl AxisState {
    pub fn switch(&mut self) {
        if self.axis_state {
            self.axis_state = false;
            self.split_axis = iced_aw::split::Axis::Horizontal
        } else {
            self.axis_state = true;
            self.split_axis = iced_aw::split::Axis::Vertical
        }
    }

    pub fn set_divider(&mut self, x: u16) {
        self.divider = Some(x)
    }
}
