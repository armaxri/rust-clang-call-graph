import assert from "assert";
import { DatabaseType } from "../../../../../../backend/Config";
import { addSuitesInSubDirsSuites } from "../../../../helper/mocha_test_helper";
import {
    getEmptyReferenceDatabase,
    prepareDatabaseEqualityTests,
} from "../../database_equality_tests";
import { FuncType } from "../../../../../../backend/database/cpp_structure";
import { assertDatabaseEquals } from "../../../../helper/database_equality";

suite("Virtual Func Call", () => {
    test("simple equality with one call", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const funcImpl = cppFile.addFuncImpl({
                    funcName: "main",
                    funcAstName: "_main",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                });
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

                const funcCall = funcImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);

                assert.ok(funcCall.isVirtual());
                assert.equal(funcCall.getFuncType(), FuncType.call);
            });
        });
    });

    test("simple get or add with one call", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const funcImpl = cppFile.addFuncImpl({
                    funcName: "main",
                    funcAstName: "_main",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                });
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

                funcImpl.getOrAddVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });
                funcImpl.getOrAddVirtualFuncCall({
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

    test("equality with multiple calls", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "multiple_simple_virtual_func_call_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "multiple_simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const funcImpl = cppFile.addFuncImpl({
                    funcName: "main",
                    funcAstName: "_main",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                });
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

                funcImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 11, column: 5 },
                        end: { line: 11, column: 8 },
                    },
                });
                funcImpl.addVirtualFuncCall({
                    func: subFuncDecl,
                    range: {
                        start: { line: 12, column: 5 },
                        end: { line: 12, column: 8 },
                    },
                });
                funcImpl.addVirtualFuncCall({
                    func: multiplyFuncDecl,
                    range: {
                        start: { line: 13, column: 5 },
                        end: { line: 13, column: 13 },
                    },
                });
                funcImpl.addVirtualFuncCall({
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

    test("no equality with multiple calls (missing implementation)", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "multiple_simple_virtual_func_call_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "multiple_simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const funcImpl = cppFile.addFuncImpl({
                    funcName: "main",
                    funcAstName: "_main",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                });
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

                funcImpl.addVirtualFuncCall({
                    func: addFuncDecl,
                    range: {
                        start: { line: 20, column: 6 },
                        end: { line: 20, column: 10 },
                    },
                });
                funcImpl.addVirtualFuncCall({
                    func: subFuncDecl,
                    range: {
                        start: { line: 13, column: 5 },
                        end: { line: 13, column: 13 },
                    },
                });
                funcImpl.addVirtualFuncCall({
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

    test("no equality with wrong call name", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const funcImpl = cppFile.addFuncImpl({
                    funcName: "main",
                    funcAstName: "_main",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                });
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

                funcImpl.addVirtualFuncCall({
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

    test("no equality with wrong location", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const funcImpl = cppFile.addFuncImpl({
                    funcName: "main",
                    funcAstName: "_main",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                });
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

                funcImpl.addVirtualFuncCall({
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

    test("Removed all database content", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_virtual_func_call_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_virtual_func_call.json"
                );
                const cppClass = cppFile.addClass("FooClass");
                const funcImpl = cppFile.addFuncImpl({
                    funcName: "main",
                    funcAstName: "_main",
                    qualType: "int (int, char **)",
                    range: {
                        start: { line: 5, column: 4 },
                        end: { line: 5, column: 9 },
                    },
                });
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

                funcImpl.addVirtualFuncCall({
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
