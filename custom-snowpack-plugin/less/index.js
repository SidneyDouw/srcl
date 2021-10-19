"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
exports.__esModule = true;
var fs_1 = __importDefault(require("fs"));
var path_1 = __importDefault(require("path"));
var less_1 = __importDefault(require("less"));
var pluginFactory = function () {
    /** A map of partially resolved imports to the files that imported them. */
    var importedByMap = new Map();
    var addImportsToMap = function (filePath, importedFile) {
        var importedBy = importedByMap.get(importedFile);
        if (importedBy) {
            importedBy.add(filePath);
        }
        else {
            importedByMap.set(importedFile, new Set([filePath]));
        }
    };
    return {
        name: '@custom-snowpack-plugin/less',
        resolve: {
            input: ['.less'],
            output: ['.css', '.less']
        },
        onChange: function (_a) {
            var _this = this;
            var filePath = _a.filePath;
            if (importedByMap.has(filePath)) {
                var importedBy = importedByMap.get(filePath);
                importedByMap["delete"](filePath);
                importedBy === null || importedBy === void 0 ? void 0 : importedBy.forEach(function (importerFilePath) {
                    _this.markChanged(importerFilePath);
                });
            }
        },
        load: function (_a) {
            var filePath = _a.filePath, isDev = _a.isDev;
            return __awaiter(this, void 0, void 0, function () {
                var fileBuffer, contents, result, output, error_1;
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, fs_1["default"].promises.readFile(filePath, 'utf-8')];
                        case 1:
                            fileBuffer = _b.sent();
                            contents = fileBuffer.toString();
                            result = { '.css': { code: '' }, '.less': { code: '' } };
                            // Emit Less
                            if (filePath.includes('/patterns/')) {
                                try {
                                    result['.less'].code = emitLess(contents);
                                }
                                catch (error) {
                                    throw new Error("Error emitting LESS:\n" + error);
                                }
                            }
                            _b.label = 2;
                        case 2:
                            _b.trys.push([2, 4, , 5]);
                            return [4 /*yield*/, emitCss(contents, filePath)];
                        case 3:
                            output = _b.sent();
                            result['.css'].code = output.css;
                            if (isDev) {
                                output.imports.forEach(function (importedFile) {
                                    addImportsToMap(filePath, importedFile);
                                });
                            }
                            return [3 /*break*/, 5];
                        case 4:
                            error_1 = _b.sent();
                            throw new Error("Error emitting CSS:\n" + error_1);
                        case 5: return [2 /*return*/, result];
                    }
                });
            });
        },
        transform: function (_a) {
            var _b;
            var fileExt = _a.fileExt, contents = _a.contents;
            return __awaiter(this, void 0, void 0, function () {
                return __generator(this, function (_c) {
                    if ((_b = this.resolve) === null || _b === void 0 ? void 0 : _b.output.includes(fileExt)) {
                        if (contents !== '') {
                            return [2 /*return*/, contents !== '' ? "/* Modified */\n\n " + contents : ''];
                        }
                    }
                    return [2 /*return*/];
                });
            });
        },
        run: function (_a) {
            var isDev = _a.isDev;
            return __awaiter(this, void 0, void 0, function () {
                return __generator(this, function (_b) {
                    console.log('run command', isDev);
                    return [2 /*return*/];
                });
            });
        },
        optimize: function (_a) {
            var buildDirectory = _a.buildDirectory;
            return __awaiter(this, void 0, void 0, function () {
                var files;
                var _this = this;
                return __generator(this, function (_b) {
                    files = getAllFiles(path_1["default"].join(buildDirectory, 'patterns')).filter(function (file) {
                        return _this.resolve.output.some(function (ext) { return file.endsWith(ext); });
                    });
                    files.forEach(function (file) {
                        var newName = file.replace('.less', '');
                        fs_1["default"].renameSync(file, newName);
                    });
                    return [2 /*return*/];
                });
            });
        }
    };
};
exports["default"] = pluginFactory;
var emitLess = function (input) {
    return input;
};
var emitCss = function (input, filePath) { return __awaiter(void 0, void 0, void 0, function () {
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0: return [4 /*yield*/, less_1["default"].render(input, {
                    filename: filePath
                })];
            case 1: return [2 /*return*/, _a.sent()];
        }
    });
}); };
var getAllFiles = function (dirPath, arrayOfFiles) {
    if (arrayOfFiles === void 0) { arrayOfFiles = []; }
    var files = fs_1["default"].readdirSync(dirPath);
    files.forEach(function (file) {
        if (fs_1["default"].statSync(dirPath + '/' + file).isDirectory()) {
            arrayOfFiles = getAllFiles(dirPath + '/' + file, arrayOfFiles);
        }
        else {
            arrayOfFiles.push(path_1["default"].join(dirPath, '/', file));
        }
    });
    return arrayOfFiles;
};
