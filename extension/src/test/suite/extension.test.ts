import * as assert from 'assert';

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
import * as vscode from 'vscode';
// import * as myExtension from '../../extension';

suite('Extension Test Suite', () => {
	console.log("Test suite");
	vscode.window.showInformationMessage('Start all tests.');

	test('Sample test', async () => {
		const extension = vscode.extensions.getExtension("Searchium.searchium");
		assert.notEqual(extension, undefined);
		await extension?.activate();

		assert.strictEqual([1, 2, 3].indexOf(5), -1);
		assert.strictEqual([1, 2, 3].indexOf(0), -1);
	});
	console.log("Test suite done");
});
