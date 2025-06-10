use atomic_float::AtomicF32;
use atomic_refcell::AtomicRefCell;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widget::{Column, Space, Text};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;
use std::time::Duration;


use crate::GainParams;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 150)
}

pub(crate) fn create(
    params: Arc<GainParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<GainEditor>(editor_state, (params, peak_meter))
}

struct GainEditor {
    params: Arc<GainParams>,
    context: Arc<dyn GuiContext>,

    peak_meter: Arc<AtomicF32>,

    gain_slider_state: Arc<AtomicRefCell<nih_widgets::param_slider::State>>,
    peak_meter_state: Arc<AtomicRefCell<nih_widgets::peak_meter::State>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for GainEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<GainParams>, Arc<AtomicF32>);

    fn new(
        (params, peak_meter): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Task<Self::Message>) {
        let editor = GainEditor {
            params,
            context,

            peak_meter,

            gain_slider_state: Default::default(),
            peak_meter_state: Default::default(),
        };

        (editor, Task::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        //_window: &mut WindowQueue,
        message: Self::Message,
    ) -> Task<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        //let slider_state = self.gain_slider_state.clone().lock().unwrap().borrow_mut();
        Column::new()
            .align_x(Alignment::Center)
            .push(
                Text::new("Gain GUI")
                    //.font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(Length::from(50))
                    .width(Length::Fill)
                    //.horizontal_alignment(alignment::Horizontal::Center)
                    //.vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(
                Text::new("Gain")
                    .height(Length::from(20))
                    .width(Length::Fill)
                    // .horizontal_alignment(alignment::Horizontal::Center)
                    // .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                nih_widgets::ParamSlider::new(self.gain_slider_state.clone(), &self.params.gain)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(Length::from(10)))
            .push(
                nih_widgets::PeakMeter::new(
                    self.peak_meter_state.clone(),
                    util::gain_to_db(self.peak_meter.load(std::sync::atomic::Ordering::Relaxed)),
                )
                .hold_time(Duration::from_millis(600)),
            )
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
    
    fn subscription(
        &self,
        _window_subs: &mut WindowSubs<Self::Message>,
    ) -> futures::Subscription<Self::Message> {
        futures::Subscription::none()
    }
    
    fn scale_policy(&self) -> baseview::WindowScalePolicy {
        baseview::WindowScalePolicy::SystemScaleFactor
    }
    
    fn handle_param_message(&self, message: nih_widgets::ParamMessage) {
        // We can't use the fancy ParamSetter here because this needs to be type erased
        let context = self.context();
        match message {
            nih_widgets::ParamMessage::BeginSetParameter(p) => unsafe { context.raw_begin_set_parameter(p) },
            nih_widgets::ParamMessage::SetParameterNormalized(p, v) => unsafe {
                context.raw_set_parameter_normalized(p, v)
            },
            nih_widgets::ParamMessage::EndSetParameter(p) => unsafe { context.raw_end_set_parameter(p) },
        }
    }
}
