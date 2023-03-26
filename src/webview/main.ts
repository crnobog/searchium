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
import { match } from "assert";

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

let queryInput: HTMLInputElement;
let filterInput: HTMLInputElement;
let matchCaseInput: HTMLInputElement;
let wholeWordInput: HTMLInputElement;
let regexInput: HTMLInputElement;

const vscode = acquireVsCodeApi();
window.addEventListener("load", main);

function main() {
    const queryForm = document.getElementById("query-form") as HTMLFormElement;
    queryForm?.addEventListener("keypress", onFormKeyUp);

    queryInput = document.getElementById("query-input") as HTMLInputElement;
    queryInput.addEventListener('change', onQueryChange);
    filterInput = document.getElementById("filter-input") as HTMLInputElement;
    filterInput.addEventListener('change', onFilterChange);
    matchCaseInput = document.getElementById("check-case-sensitive") as HTMLInputElement;
    matchCaseInput.addEventListener('change', onMatchCaseChange);
    wholeWordInput = document.getElementById("check-whole-word") as HTMLInputElement;
    wholeWordInput.addEventListener('change', onWholeWordChange);
    regexInput = document.getElementById("check-regex") as HTMLInputElement;
    regexInput.addEventListener('change', onRegexChange);
}

function onQueryChange() {
    vscode.postMessage({ command: "setQuery", text: queryInput.value });
}
function onFilterChange() {
    vscode.postMessage({ command: "setFilter", text: filterInput.value });
}
function onMatchCaseChange() {
    vscode.postMessage({ command: "setMatchCase", value: matchCaseInput.checked });
}
function onWholeWordChange() {
    vscode.postMessage({ command: "setWholeWord", value: wholeWordInput.checked });
}
function onRegexChange() {
    vscode.postMessage({ command: "setRegex", value: regexInput.checked });
}

function onFormKeyUp(e: KeyboardEvent) {
    if (e.key === "Enter") {
        vscode.postMessage({ command: "query", text: queryInput.value });
    }
}

export { };