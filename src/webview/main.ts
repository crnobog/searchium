import {
    provideVSCodeDesignSystem,
    Checkbox,
    DataGrid,
    vsCodeBadge,
    vsCodeButton,
    vsCodeCheckbox,
    vsCodeDataGrid,
    vsCodeDataGridCell,
    vsCodeDataGridRow,
    vsCodeDivider,
    vsCodeDropdown,
    vsCodeLink,
    vsCodeOption,
    vsCodePanels,
    vsCodePanelTab,
    vsCodePanelView,
    vsCodeProgressRing,
    vsCodeRadio,
    vsCodeRadioGroup,
    vsCodeTag,
    vsCodeTextArea,
    vsCodeTextField,
} from "@vscode/webview-ui-toolkit";
import * as vscodeUi from "@vscode/webview-ui-toolkit";

// In order to use the Webview UI Toolkit web components they
// must be registered with the browser (i.e. webview) using the
// syntax below.
provideVSCodeDesignSystem().register(
    vsCodeBadge(),
    vsCodeButton(),
    vsCodeCheckbox(),
    vsCodeDataGrid(),
    vsCodeDataGridCell(),
    vsCodeDataGridRow(),
    vsCodeDivider(),
    vsCodeDropdown(),
    vsCodeLink(),
    vsCodeOption(),
    vsCodePanels(),
    vsCodePanelTab(),
    vsCodePanelView(),
    vsCodeProgressRing(),
    vsCodeRadio(),
    vsCodeRadioGroup(),
    vsCodeTag(),
    vsCodeTextArea(),
    vsCodeTextField()
);

declare function acquireVsCodeApi(): {
    postMessage(msg: any): void,
};

const vscode = acquireVsCodeApi();
window.addEventListener("load", main);

function main() {
    const queryForm = document.getElementById("query-form") as HTMLFormElement;
    queryForm?.addEventListener("keypress", onFormKeyUp);
}

function onFormKeyUp(e: KeyboardEvent) {
    if (e.key === "Enter") {
        const queryInput = document.getElementById("query-input") as HTMLInputElement;
        vscode.postMessage({ command: "query", text: queryInput.value });
    }
}

export { };