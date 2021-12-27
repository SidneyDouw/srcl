"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
exports.__esModule = true;
exports.getRootDir = void 0;
var fs_1 = __importDefault(require("fs"));
var path_1 = __importDefault(require("path"));
var getRootDir = function () {
    for (var _i = 0, _a = module.paths; _i < _a.length; _i++) {
        var p = _a[_i];
        if (fs_1["default"].existsSync(p)) {
            return path_1["default"].dirname(p);
        }
    }
    throw new Error('Could not find project root directory');
};
exports.getRootDir = getRootDir;
