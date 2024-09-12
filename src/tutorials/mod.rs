pub mod editor;
pub mod output;
pub mod console;

use leptos::*;

use editor::TutorialEditorArea;
use output::TutorialOutputArea;
use console::TutorialConsoleArea;

#[component]
pub fn TutorialPage() -> impl IntoView {

    view! {
        <div class="tutorial">
            <div class="toolbar">
                <button>保存</button>
                <button>运行</button>
            </div>
            <div class="editor_area">
                <div class="text_area">
                    <TutorialEditorArea />
                </div>
                <div class="console_area">
                    <TutorialConsoleArea />
                </div>
            </div>
            <div class="middlebar"></div>
            <div class="output_area">
                <TutorialOutputArea />
            </div>
        </div>
    }
}
