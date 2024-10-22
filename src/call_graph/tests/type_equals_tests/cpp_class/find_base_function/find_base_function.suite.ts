import assert from "assert";
import { DatabaseType } from "../../../../../../backend/Config";
import { addSuitesInSubDirsSuites } from "../../../../helper/mocha_test_helper";
import { openNewDatabase } from "../../database_equality_tests";
import { assertFuncEquals } from "../../../../helper/database_equality";

suite("Find Base function", () => {
    addSuitesInSubDirsSuites(__dirname);

    suite("Find parent class func declaration", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const database = openNewDatabase(__dirname, testData);

                const cppFile = database.getOrAddCppFile(
                    "simple_parent_cpp_class.json"
                );
                const cppClass = cppFile.addClass("FooClass");

                const virtualFuncDecl = cppClass.addVirtualFuncDecl({
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcName: "add",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });

                database.writeDatabase();

                assertFuncEquals(
                    cppClass.findBaseFunction("add", "int (int, int)")!,
                    virtualFuncDecl
                );
            });
        });
    });

    suite("Find parent class func implementation", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const database = openNewDatabase(__dirname, testData);

                const cppFile = database.getOrAddCppFile(
                    "simple_parent_cpp_class.json"
                );
                const cppClass = cppFile.addClass("FooClass");

                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcName: "add",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });

                database.writeDatabase();

                assertFuncEquals(
                    cppClass.findBaseFunction("add", "int (int, int)")!,
                    virtualFuncImpl
                );
            });
        });
    });
});
