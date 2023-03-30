import {
    provideVSCodeDesignSystem,
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
    Option
} from "@vscode/webview-ui-toolkit";
import * as FromWebView from '../shared/fromDetailsWebview';
import * as ToWebView from '../shared/toDetailsWebview';

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

let progressContainer: HTMLElement;
let detailsContainer: HTMLElement;
let projectsDropdown: HTMLSelectElement;

let detailsNumFiles: HTMLElement;
let detailsNumDirectories: HTMLElement;
let detailsNumSearchableFiles: HTMLElement;
let detailsSizeSearchableFiles: HTMLElement;
let detailsNumBinaryFiles: HTMLElement;
let detailsSizeBinaryFiles: HTMLElement;

let projects: ToWebView.ProjectDetails[] = [];

window.addEventListener("load", () => {
    console.log("details view loaded");
    progressContainer = document.getElementById('progress-container') as HTMLElement;
    detailsContainer = document.getElementById('details-container') as HTMLElement;
    projectsDropdown = document.getElementById('projects-dropdown') as HTMLSelectElement;
    projectsDropdown.addEventListener('change', onProjectChanged);

    detailsNumFiles = document.getElementById("details-num-files") as HTMLElement;
    detailsNumDirectories = document.getElementById("details-num-directories") as HTMLElement;
    detailsNumSearchableFiles = document.getElementById("details-num-searchable-files") as HTMLElement;
    detailsSizeSearchableFiles = document.getElementById("details-size-searchable-files") as HTMLElement;
    detailsNumBinaryFiles = document.getElementById("details-num-binary-files") as HTMLElement;
    detailsSizeBinaryFiles = document.getElementById("details-size-binary-files") as HTMLElement;

    vscode.postMessage(<FromWebView.ReadyMessage>{ type: 'ready' });
});
window.addEventListener("message", (event: { data: ToWebView.Message }) => {
    let msg = event.data;
    switch (msg.type) {
        case 'details':
            console.log("adding hidden to progress container and removing it from details container");
            progressContainer.remove();
            detailsContainer.classList.remove("hidden");

            projects = msg.projects;
            let options: Node[] = [];
            for (let p of projects) {
                let option = new Option(p.rootPath);
                options.push(option);
            }
            projectsDropdown.replaceChildren(...options);
            setProjectDetails(projects[0]);
            break;
    }
});

function onProjectChanged() {
    let p = projects[projectsDropdown.selectedIndex];
    setProjectDetails(p);
}

function setProjectDetails(p: ToWebView.ProjectDetails) {
    console.log(`Project changed to ${p.rootPath}`);
    detailsNumFiles.textContent = p.numFiles;
    detailsNumDirectories.textContent = p.numDirectories;
    detailsNumSearchableFiles.textContent = p.numSearchableFiles;
    detailsSizeSearchableFiles.textContent = `${p.searchableFilesMB.toFixed(2)} MB`;
    detailsNumBinaryFiles.textContent = p.numBinaryFiles;
    detailsSizeBinaryFiles.textContent = `${p.binaryFilesMB.toFixed(2)} MB`;
}


export { };