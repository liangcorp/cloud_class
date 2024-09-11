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
        // <div align="right" style="height:30px">
        //     <a href="javascript:close_window();">
        //         关闭实验室
        //     </a>
        // </div>
        // <hr />
        <table class="tutorial">
            <tr>
                <td class="tutorial">
                    <div class="text_area">
                        <TutorialEditorArea />
                    </div>
                    <div class="console_area">
                        <TutorialConsoleArea />
                    </div>
                </td>
                <td class="tutorial"><TutorialOutputArea /></td>
            </tr>
        </table>
    }
}
