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
    Option,
    DataGrid,
    TextArea,
    TextField
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
    postMessage(msg: FromWebView.Message): void,
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

let extensionsGrid: DataGrid;
let largeFilesGrid: DataGrid;
let binariesGrid: DataGrid;
let largeBinariesGrid: DataGrid;

let ignorePathsPath: TextField;
let ignorePathsSectionName: TextField;
let ignorePathsContents: TextArea;

let ignoreFilesPath: TextField;
let ignoreFilesSectionName: TextField;
let ignoreFilesContents: TextArea;

let includeFilesPath: TextField;
let includeFilesSectionName: TextField;
let includeFilesContents: TextArea;

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

    extensionsGrid = document.getElementById("grid-extensions") as DataGrid;
    largeFilesGrid = document.getElementById("grid-large-files") as DataGrid;
    binariesGrid = document.getElementById("grid-binary-extensions") as DataGrid;
    largeBinariesGrid = document.getElementById("grid-large-binaries") as DataGrid;

    ignorePathsPath = document.getElementById("ignore-paths-path") as TextField;
    ignorePathsSectionName = document.getElementById("ignore-paths-name") as TextField;
    ignorePathsContents = document.getElementById("ignore-paths-contents") as TextArea;

    ignoreFilesPath = document.getElementById("ignore-files-path") as TextField;
    ignoreFilesSectionName = document.getElementById("ignore-files-name") as TextField;
    ignoreFilesContents = document.getElementById("ignore-files-contents") as TextArea;

    includeFilesPath = document.getElementById("include-files-path") as TextField;
    includeFilesSectionName = document.getElementById("include-files-name") as TextField;
    includeFilesContents = document.getElementById("include-files-contents") as TextArea;

    vscode.postMessage({ type: 'ready' });

    if (projects.length > 0) {
        onProjectsAvailable();
    }
});
window.addEventListener("message", (event: { data: ToWebView.Message }) => {
    const msg = event.data;
    switch (msg.type) {
        case 'details':
            console.log("received details");
            projects = msg.projects;
            onProjectsAvailable();
            break;
    }
});


function onProjectsAvailable(): void {
    console.log("adding hidden to progress container and removing it from details container");
    progressContainer.remove();
    detailsContainer.classList.remove("hidden");

    const options: Node[] = [];
    for (const p of projects) {
        const option = new Option(p.rootPath);
        options.push(option);
    }
    projectsDropdown.replaceChildren(...options);
    setProjectDetails(projects[0]);
}

function onProjectChanged(): void {
    const p = projects[projectsDropdown.selectedIndex];
    setProjectDetails(p);
}

function setProjectDetails(p: ToWebView.ProjectDetails): void {
    console.log(`Project changed to ${p.rootPath}`);
    detailsNumFiles.textContent = p.numFiles;
    detailsNumDirectories.textContent = p.numDirectories;
    detailsNumSearchableFiles.textContent = p.numSearchableFiles;
    detailsSizeSearchableFiles.textContent = `${p.searchableFilesMB.toFixed(2)} MB`;
    detailsNumBinaryFiles.textContent = p.numBinaryFiles;
    detailsSizeBinaryFiles.textContent = `${p.binaryFilesMB.toFixed(2)} MB`;

    /* eslint-disable @typescript-eslint/naming-convention */
    extensionsGrid.rowsData = p.searchableByExtension.map((x: ToWebView.FileByExtensionDetails) => {
        return {
            "File Extension": x.extension,
            "Count": x.count,
            "Total Size": x.size,
        };
    });
    largeFilesGrid.rowsData = p.largeFiles.map((x: ToWebView.LargeFileDetails) => {
        return {
            "Path": x.path,
            "Size": x.size
        };
    });
    binariesGrid.rowsData = p.binaryByExtension.map((x: ToWebView.FileByExtensionDetails) => {
        return {
            "File Extension": x.extension,
            "Count": x.count,
            "Total Size": x.size,
        };
    });
    largeBinariesGrid.rowsData = p.largeBinaries.map((x: ToWebView.LargeFileDetails) => {
        return {
            "Path": x.path,
            "Size": x.size
        };
    });
    /* eslint-enable @typescript-eslint/naming-convention */

    ignorePathsPath.value = p.ignorePaths.path;
    ignorePathsSectionName.value = p.ignorePaths.name;
    ignorePathsContents.value = p.ignorePaths.contents;

    ignoreFilesPath.value = p.ignoreFiles.path;
    ignoreFilesSectionName.value = p.ignoreFiles.name;
    ignoreFilesContents.value = p.ignoreFiles.contents;

    includeFilesPath.value = p.includeFiles.path;
    includeFilesSectionName.value = p.includeFiles.name;
    includeFilesContents.value = p.includeFiles.contents;
}


export { };