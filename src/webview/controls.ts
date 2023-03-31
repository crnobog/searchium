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
import * as FromWebView from '../shared/fromControlsWebview';
import * as ToWebView from '../shared/toControlsWebview';

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

let queryForm: HTMLElement;
let queryInput: HTMLInputElement;
let filterInput: HTMLInputElement;
let matchCaseInput: HTMLInputElement;
let wholeWordInput: HTMLInputElement;
let regexInput: HTMLInputElement;
let searchButton: HTMLInputElement;

let tagNumFiles: HTMLElement;
let tagMemory: HTMLElement;
let tagState: HTMLElement;

const vscode = acquireVsCodeApi();
window.addEventListener("load", () => {
    queryForm = document.getElementById("query-form") as HTMLElement;
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
    searchButton = document.getElementById("query-execute") as HTMLInputElement;
    searchButton.addEventListener('click', onExecuteClick);

    tagNumFiles = document.getElementById("tag-num-files")!;
    tagMemory = document.getElementById("tag-memory-usage")!;
    tagState = document.getElementById("tag-index-state")!;

    console.log("Controls view ready");
    vscode.postMessage({ command: 'ready' });
});
window.addEventListener("message", (event: { data: ToWebView.Message }) => {
    let msg = event.data;
    switch (msg.type) {
        case 'nostatus':
            console.log(`Received nostatus`);
            queryForm.classList.remove("db-available");
            break;
        case 'status':
            console.log(`Received status`);
            queryForm.classList.add("db-available");
            tagNumFiles.textContent = msg.numFiles;
            tagMemory.textContent = msg.memory;
            tagState.textContent = msg.state;
            break;
        case "options":
            console.log(`Received full options with qiuery ${msg.query}`);
            queryInput.value = msg.query;
            filterInput.value = msg.pathFilter;
            matchCaseInput.checked = msg.matchCase;
            wholeWordInput.checked = msg.wholeWord;
            regexInput.checked = msg.regex;
            break; 
        case 'setQuery':
            console.log("Received setQuery");
            queryInput.value = msg.query;
            break;
        case 'focus':
            console.log("Received focus");
            queryInput.focus();
            break;
        case 'setMatchCase':
            console.log("Received setMatchCase");
            matchCaseInput.checked = msg.matchCase;
            break;
        case 'setWholeWord':
            console.log("Received setWholeWord");
            wholeWordInput.checked = msg.wholeWord;
            break;
        case 'setRegex':
            console.log("Received setRegex");
            regexInput.checked = msg.regex;
            break;
    }
});

function main() {
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
function onExecuteClick() {
    vscode.postMessage(createQueryCommand());
}
function onFormKeyUp(e: KeyboardEvent) {
    if (e.key === "Enter") {
        vscode.postMessage(createQueryCommand());
    }
}

function createQueryCommand() {
    return {
        command: "execute",
        query: queryInput.value,
        pathFilter: filterInput.value,
        matchCase: matchCaseInput.checked,
        wholeWord: wholeWordInput.checked,
        regex: regexInput.checked
    };
}


export { };