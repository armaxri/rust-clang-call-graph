import assert from "assert";
import { DatabaseType } from "../../../../../../backend/Config";
import { addSuitesInSubDirsSuites } from "../../../../helper/mocha_test_helper";
import {
    getEmptyReferenceDatabase,
    prepareDatabaseEqualityTests,
} from "../../database_equality_tests";
import { assertDatabaseEquals } from "../../../../helper/database_equality";

suite("Virtual Func Call", () => {
    addSuitesInSubDirsSuites(__dirname);

    suite("Simple equality with one call", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const addFuncDecl = cppClass.addVirtualFuncDecl({
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcName: "add",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    funcName: "base",
                    funcAstName: "__ZN3foo6baseEii",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                    baseFuncAstName: "__ZN3foo6baseEii",
                });

                virtualFuncImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });
    addSuitesInSubDirsSuites(__dirname);

    suite("Simple get or add with one call", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const addFuncDecl = cppClass.addVirtualFuncDecl({
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcName: "add",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    funcName: "base",
                    funcAstName: "__ZN3foo6baseEii",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                    baseFuncAstName: "__ZN3foo6baseEii",
                });

                virtualFuncImpl.getOrAddVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });
                virtualFuncImpl.getOrAddVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    suite("Equality with multiple calls", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "multiple_simple_virtual_func_call_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile(
                    "multiple_simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const addFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "add",
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                const subFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "sub",
                    baseFuncAstName: "__ZN3foo3subEii",
                    funcAstName: "__ZN3foo3subEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 12, column: 5 },
                        end: { line: 12, column: 8 },
                    },
                });
                const multiplyFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "multiply",
                    baseFuncAstName: "__ZN3foo8multiplyEii",
                    funcAstName: "__ZN3foo8multiplyEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 13, column: 5 },
                        end: { line: 13, column: 13 },
                    },
                });
                const divideFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "divide",
                    baseFuncAstName: "__ZN3foo6divideEii",
                    funcAstName: "__ZN3foo6divideEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 14, column: 5 },
                        end: { line: 14, column: 11 },
                    },
                });
                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    funcName: "base",
                    funcAstName: "__ZN3foo6baseEii",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                    baseFuncAstName: "__ZN3foo6baseEii",
                });

                virtualFuncImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                virtualFuncImpl.addVirtualFuncCall({
                    func: subFuncDecl,
                    range: {
                        start: { line: 12, column: 5 },
                        end: { line: 12, column: 8 },
                    },
                });
                virtualFuncImpl.addVirtualFuncCall({
                    func: multiplyFuncDecl,
                    range: {
                        start: { line: 13, column: 5 },
                        end: { line: 13, column: 13 },
                    },
                });
                virtualFuncImpl.addVirtualFuncCall({
                    func: divideFuncDecl,
                    range: {
                        start: { line: 14, column: 5 },
                        end: { line: 14, column: 11 },
                    },
                });

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    suite("No equality with multiple calls (missing implementation)", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "multiple_simple_virtual_func_call_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile(
                    "multiple_simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const addFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "add",
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                const subFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "sub",
                    baseFuncAstName: "__ZN3foo3subEii",
                    funcAstName: "__ZN3foo3subEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 12, column: 5 },
                        end: { line: 12, column: 8 },
                    },
                });
                const multiplyFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "multiply",
                    baseFuncAstName: "__ZN3foo8multiplyEii",
                    funcAstName: "__ZN3foo8multiplyEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 13, column: 5 },
                        end: { line: 13, column: 13 },
                    },
                });
                const divideFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "divide",
                    baseFuncAstName: "__ZN3foo6divideEii",
                    funcAstName: "__ZN3foo6divideEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 14, column: 5 },
                        end: { line: 14, column: 11 },
                    },
                });
                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    funcName: "base",
                    funcAstName: "__ZN3foo6baseEii",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                    baseFuncAstName: "__ZN3foo6baseEii",
                });

                virtualFuncImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });
                virtualFuncImpl.addVirtualFuncCall({
                    func: subFuncDecl,
                    range: {
                        start: { line: 13, column: 5 },
                        end: { line: 13, column: 13 },
                    },
                });
                virtualFuncImpl.addVirtualFuncCall({
                    func: divideFuncDecl,
                    range: {
                        start: { line: 14, column: 5 },
                        end: { line: 14, column: 11 },
                    },
                });

                database.writeDatabase();

                assert.throws(() =>
                    assertDatabaseEquals(database, referenceDatabase)
                );
            });
        });
    });

    suite("No equality with wrong call name", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const addFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "multiply",
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    funcName: "base",
                    funcAstName: "__ZN3foo6baseEii",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                    baseFuncAstName: "__ZN3foo6baseEii",
                });

                virtualFuncImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });

                database.writeDatabase();

                assert.throws(() =>
                    assertDatabaseEquals(database, referenceDatabase)
                );
            });
        });
    });

    suite("No equality with wrong location", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const addFuncDecl = cppClass.addVirtualFuncDecl({
                    funcName: "add",
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    funcName: "base",
                    funcAstName: "__ZN3foo6baseEii",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                    baseFuncAstName: "__ZN3foo6baseEii",
                });

                virtualFuncImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 30, column: 10 },
                    },
                });

                database.writeDatabase();

                assert.throws(() =>
                    assertDatabaseEquals(database, referenceDatabase)
                );
            });
        });
    });

    suite("Removed all database content", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const addFuncDecl = cppClass.addVirtualFuncDecl({
                    baseFuncAstName: "__ZN3foo3addEii",
                    funcName: "add",
                    funcAstName: "__ZN3foo3addEii",
                    qualType: "int (int, int)",
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                const virtualFuncImpl = cppClass.addVirtualFuncImpl({
                    funcName: "base",
                    funcAstName: "__ZN3foo6baseEii",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                    baseFuncAstName: "__ZN3foo6baseEii",
                });

                virtualFuncImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);

                database.removeCppFileAndDependingContent(cppFile.getName());
                database.writeDatabase();
                assertDatabaseEquals(database, getEmptyReferenceDatabase());
            });
        });
    });
});
