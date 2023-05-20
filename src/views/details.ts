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

let roots: ToWebView.DatabaseDetailsRoot[] = [];

let extensionsGrid: DataGrid;
let largeFilesGrid: DataGrid;
let binariesGrid: DataGrid;
let largeBinariesGrid: DataGrid;

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

    vscode.postMessage({ type: 'ready' });

    if (roots.length > 0) {
        onProjectsAvailable();
    }
});
window.addEventListener("message", (event: { data: ToWebView.Message }) => {
    const msg = event.data;
    switch (msg.type) {
        case 'details':
            console.log("received details");
            roots = msg.roots;
            onProjectsAvailable();
            break;
    }
});


function onProjectsAvailable(): void {
    console.log("adding hidden to progress container and removing it from details container");
    progressContainer.remove();
    detailsContainer.classList.remove("hidden");

    const options: Node[] = [];
    for (const r of roots) {
        const option = new Option(r.rootPath);
        options.push(option);
    }
    projectsDropdown.replaceChildren(...options);
    setProjectDetails(roots[0]);
}

function onProjectChanged(): void {
    const p = roots[projectsDropdown.selectedIndex];
    setProjectDetails(p);
}

function setProjectDetails(p: ToWebView.DatabaseDetailsRoot): void {
    console.log(`Project changed to ${p.rootPath}`);
    detailsNumFiles.textContent = p.numFilesScanned;
    detailsNumDirectories.textContent = p.numDirectoriesScanned;
    detailsNumSearchableFiles.textContent = p.numSearchableFiles;
    detailsSizeSearchableFiles.textContent = `${p.searchableFilesMB.toFixed(2)} MB`;
    detailsNumBinaryFiles.textContent = p.numBinaryFiles;
    detailsSizeBinaryFiles.textContent = `${p.binaryFilesMB.toFixed(2)} MB`;

    /* eslint-disable @typescript-eslint/naming-convention */
    extensionsGrid.rowsData = p.searchableFilesByExtension.map((x: ToWebView.FilesByExtensionDetails) => {
        return {
            "File Extension": x.extension,
            "Count": x.count,
            "Total Size (MB)": x.mb.toFixed(2),
        };
    });
    largeFilesGrid.rowsData = p.largeFiles.map((x: ToWebView.LargeFileDetails) => {
        return {
            "Path": x.path,
            "Size (MB)": x.sizeMb.toFixed(2)
        };
    });
    binariesGrid.rowsData = p.binaryFilesByExtension.map((x: ToWebView.FilesByExtensionDetails) => {
        return {
            "File Extension": x.extension,
            "Count": x.count,
            "Total Size (MB)": x.mb.toFixed(2),
        };
    });
    largeBinariesGrid.rowsData = p.largeBinaries.map((x: ToWebView.LargeFileDetails) => {
        return {
            "Path": x.path,
            "Size (MB)": x.sizeMb.toFixed(2)
        };
    });
    /* eslint-enable @typescript-eslint/naming-convention */
}


export { };