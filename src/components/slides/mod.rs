mod title_slide;
mod timelines_slide;
mod pre_cm;
mod cm_overview;
mod cfengine;
mod puppet;
mod chef;
mod ansible;
mod iac_overview;
mod cloudformation;
mod arm;
mod terraform;
mod bicep;
mod tf_vs_bicep;

use leptos::*;
use title_slide::TitleSlide;
use timelines_slide::TimelinesSlide;
use pre_cm::PreCmSlide;
use cm_overview::CmOverviewSlide;
use cfengine::CfengineSlide;
use puppet::PuppetSlide;
use chef::ChefSlide;
use ansible::AnsibleSlide;
use iac_overview::IacOverviewSlide;
use cloudformation::CloudFormationSlide;
use arm::ArmSlide;
use terraform::TerraformSlide;
use bicep::BicepSlide;
use tf_vs_bicep::TfVsBicepSlide;

/// Total number of slides is MAX_SLIDE + 1  (slides 0 … MAX_SLIDE)
pub const MAX_SLIDE: usize = 13;

#[component]
pub fn SlideDispatch() -> impl IntoView {
    let slide = use_context::<RwSignal<usize>>().expect("slide signal");

    view! {
        <div class="slide-viewport">
            {move || match slide.get() {
                0  => view! { <TitleSlide /> }.into_view(),
                1  => view! { <TimelinesSlide /> }.into_view(),
                2  => view! { <PreCmSlide /> }.into_view(),
                3  => view! { <CmOverviewSlide /> }.into_view(),
                4  => view! { <CfengineSlide /> }.into_view(),
                5  => view! { <PuppetSlide /> }.into_view(),
                6  => view! { <ChefSlide /> }.into_view(),
                7  => view! { <AnsibleSlide /> }.into_view(),
                8  => view! { <IacOverviewSlide /> }.into_view(),
                9  => view! { <CloudFormationSlide /> }.into_view(),
                10 => view! { <ArmSlide /> }.into_view(),
                11 => view! { <TerraformSlide /> }.into_view(),
                12 => view! { <BicepSlide /> }.into_view(),
                13 => view! { <TfVsBicepSlide /> }.into_view(),
                _  => view! { <TitleSlide /> }.into_view(),
            }}
        </div>
    }
}
