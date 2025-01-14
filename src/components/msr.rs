use crate::misc::prec;
use crate::styles::Styled;
use crate::{styles, App, Message};
use iced::widget::{column, row, text, Column, Container};
use iced::widget::{Row, Text};
use iced::Color;

type ElementSimple<'a> = iced::Element<'a, <App as iced::Application>::Message>;

impl crate::tabs::cpu::Cpu {
    pub fn generate_cpu<'a>(&self) -> ElementSimple<'a> {
        let data = &self.msr;

        let mut cache_column: Column<'a, Message> = column![text("Cache").size(31)];
        for c in &data.cache {
            let title = format!("Cache L{} {:13}", c.level, c.cache_type);
            let title: Text<'static> = text(title).size(21).style(Color::new(0.3, 0.8, 0.3, 1.0));
            let c_data = format!("{} kB", c.size as f64 / 1024.);
            let c_data: Text<'static> = text(c_data).size(21);
            cache_column = cache_column.push(row![title, c_data].padding(5).spacing(10));
        }
        let cache_section =
            Container::new(cache_column).padding_style(14, styles::boxes::surround_with_box());

        let len = data.per_core_freq.len() as u64;
        let mut freq_layout: Row<'a, Message> = row![].spacing(20);
        let mut id = 0u64;
        let mut freq = 0u64;

        while id < len {
            let mut col: Column<'a, Message> = column![].spacing(6);
            for _ in 0..6 {
                if id >= len {
                    break;
                }
                freq += data.per_core_freq[id as usize];
                col = col.push(
                    text(format!(
                        "core {}: {}MHz",
                        id, data.per_core_freq[id as usize]
                    ))
                    .size(16),
                );
                id += 1;
            }
            freq_layout = freq_layout.push(col);
        }
        freq = freq / len;

        let freq_section =
            Container::new(column![text("Per Core Frequency").size(31), freq_layout])
                .padding_style(14, styles::boxes::surround_with_box());

        let mut temp_txt = text(format!(
            "Temperature: {: >7}°C",
            prec(data.temperature as f64)
        ))
        .size(20);

        let avg_freq: Text<'a> = text(format!("Avg Frequency: {: >7}MHz", freq)).size(20);
        if data.temperature > 50. {
            temp_txt = temp_txt.style(Color::new(1., 0., 0., 1.));
        };

        let mut usage_txt: Text<'a> = text(format!("Util: {: >7}%", prec(data.util))).size(20);
        if data.util > 50. {
            usage_txt = usage_txt.style(Color::new(1., 0.1, 0.5, 1.));
        };

        let volt: Text<'a> = text(format!("Power: {: >7}W", prec(data.package_power))).size(20);
        let pwr: Text<'a> = text(format!("Voltage: {: >7}V", prec(data.voltage))).size(20);

        let col1 = Column::new()
            .spacing(10)
            .push(text(format!("Cores: {:4}", data.cores)).size(20))
            .push(temp_txt)
            .push(volt)
            .push(avg_freq);
        let col2 = Column::new()
            .spacing(10)
            .push(text(format!("Threads: {:4}", data.threads)).size(20))
            .push(usage_txt)
            .push(pwr);

        let row = row![col1, col2].spacing(35);

        let memory =
            Container::new(Column::new()).padding_style(14, styles::boxes::surround_with_box());

        Container::new(
            Column::new()
                .push(styles::title::title(&data.name))
                .push(row)
                .push(cache_section)
                .push(freq_section)
                .push(memory)
                .spacing(10),
        )
        .padding_style(14, styles::boxes::surround_with_box())
        .into()
    }
}
