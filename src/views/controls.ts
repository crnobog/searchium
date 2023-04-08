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
    Button
} from "@vscode/webview-ui-toolkit";
import * as FromWebView from '../shared/fromControlsWebview';
import * as ToWebView from '../shared/toControlsWebview';
import { assertUnreachable } from '../utils';

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

let prevButton: Button;
let nextButton: Button;
let searchButton: Button;

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
    searchButton = document.getElementById("query-execute") as Button;
    searchButton.addEventListener('click', onExecuteClick);
    prevButton = document.getElementById("query-prev") as Button;
    prevButton.addEventListener('click', onPrevClick);
    nextButton = document.getElementById("query-next") as Button;
    nextButton.addEventListener('click', onNextClick);

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
        case 'setHistoryControls':
            prevButton.disabled = !msg.prevEnabled;
            nextButton.disabled = !msg.nextEnabled;
            break;
        default:
            assertUnreachable(msg);
    }
});

function postMessage(msg: FromWebView.Message) {
    vscode.postMessage(msg);
}

function onQueryChange() {
    postMessage({ command: "setQuery", text: queryInput.value });
}
function onFilterChange() {
    postMessage({ command: "setFilter", text: filterInput.value });
}
function onMatchCaseChange() {
    postMessage({ command: "setMatchCase", value: matchCaseInput.checked });
}
function onWholeWordChange() {
    postMessage({ command: "setWholeWord", value: wholeWordInput.checked });
}
function onRegexChange() {
    postMessage({ command: "setRegex", value: regexInput.checked });
}
function onExecuteClick() {
    postMessage(createQueryCommand());
}
function onPrevClick() {
    postMessage({ command: "prevQuery" });
}
function onNextClick() {
    postMessage({ command: "nextQuery" });
}
function onFormKeyUp(e: KeyboardEvent) {
    if (e.key === "Enter") {
        vscode.postMessage(createQueryCommand());
    }
}

function createQueryCommand(): FromWebView.Execute {
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