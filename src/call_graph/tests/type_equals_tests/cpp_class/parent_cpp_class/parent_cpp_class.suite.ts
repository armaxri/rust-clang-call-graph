import assert from "assert";
import { DatabaseType } from "../../../../../../backend/Config";
import { addSuitesInSubDirsSuites } from "../../../../helper/mocha_test_helper";
import {
    getEmptyReferenceDatabase,
    openDatabase,
    prepareDatabaseEqualityTests,
} from "../../database_equality_tests";
import {
    assertCppClassEquals,
    assertDatabaseEquals,
} from "../../../../helper/database_equality";

suite("Parent Cpp Class", () => {
    test("equality with simple parent class", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_parent_cpp_class_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_parent_cpp_class.json"
                );
                const parentClass = cppFile.addClass("ParentClass");
                const childClass = cppFile.addClass("ChildClass");

                childClass.addParentClass(parentClass);

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    test("Get or add with simple parent class", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_parent_cpp_class_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_parent_cpp_class.json"
                );
                const parentClass = cppFile.addClass("ParentClass");
                const childClass = cppFile.addClass("ChildClass");

                childClass.getOrAddParentClass(parentClass);
                childClass.getOrAddParentClass(parentClass);

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    test("equality with multiple simple parent class", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "multiple_simple_parent_cpp_class_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "multiple_simple_parent_cpp_class.json"
                );
                const parentClass1 = cppFile.addClass("ParentClass1");
                const parentClass2 = cppFile.addClass("ParentClass2");
                const parentClass3 = cppFile.addClass("ParentClass3");
                const parentClass4 = cppFile.addClass("ParentClass4");
                const childClass = cppFile.addClass("ChildClass");

                childClass.addParentClass(parentClass1);
                childClass.addParentClass(parentClass2);
                childClass.addParentClass(parentClass3);
                childClass.addParentClass(parentClass4);

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    suite(
        "No equality with multiple simple parent class (missing parent)",
        () => {
            [DatabaseType.lowdb, DatabaseType.sqlite].forEach((DatabaseType.sqlite) => {
                test(`${DatabaseType[DatabaseType.sqlite]}`, () => {
                    const [database, referenceDatabase] =
                        prepareDatabaseEqualityTests(
                            __dirname,
                            "multiple_simple_parent_cpp_class_expected_db.json",
                            DatabaseType.sqlite
                        );
                    const cppFile = database.getOrAddCppFile(
                        "multiple_simple_parent_cpp_class.json"
                    );
                    const parentClass1 = cppFile.addClass("ParentClass1");
                    const parentClass2 = cppFile.addClass("ParentClass2");
                    cppFile.addClass("ParentClass3");
                    const parentClass4 = cppFile.addClass("ParentClass4");
                    const childClass = cppFile.addClass("ChildClass");

                    childClass.addParentClass(parentClass1);
                    childClass.addParentClass(parentClass2);
                    childClass.addParentClass(parentClass4);

                    database.writeDatabase();

                    assert.throws(() =>
                        assertDatabaseEquals(database, referenceDatabase)
                    );
                });
            });
        }
    );

    test("equality with multiple chained parent class", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "parent_cpp_class_chain_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "parent_cpp_class_chain.json"
                );
                const grandParentClass = cppFile.addClass("GrandParentClass");
                const parentClass = cppFile.addClass("ParentClass");
                parentClass.addParentClass(grandParentClass);
                const childClass = cppFile.addClass("ChildClass");
                childClass.addParentClass(parentClass);

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    suite(
        "No equality with multiple chained parent class (missing middle class)",
        () => {
            [DatabaseType.lowdb, DatabaseType.sqlite].forEach((DatabaseType.sqlite) => {
                test(`${DatabaseType[DatabaseType.sqlite]}`, () => {
                    const [database, referenceDatabase] =
                        prepareDatabaseEqualityTests(
                            __dirname,
                            "parent_cpp_class_chain_expected_db.json",
                            DatabaseType.sqlite
                        );
                    const cppFile = database.getOrAddCppFile(
                        "parent_cpp_class_chain.json"
                    );
                    const grandParentClass =
                        cppFile.addClass("GrandParentClass");
                    const parentClass = cppFile.addClass("ParentClass");
                    parentClass.addParentClass(grandParentClass);
                    const childClass = cppFile.addClass("ChildClass");
                    childClass.addParentClass(grandParentClass);

                    database.writeDatabase();

                    assert.throws(() =>
                        assertDatabaseEquals(database, referenceDatabase)
                    );
                });
            });
        }
    );

    test("Get parent classes", () => {

                const database = prepareDatabaseEqualityTests(
                    __dirname,
                    "simple_parent_cpp_class_expected_db.json",
                    DatabaseType.sqlite
                )[0];
                const cppFile = database.getOrAddCppFile(
                    "simple_parent_cpp_class.json"
                );
                const parentClass = cppFile.addClass("ParentClass");
                const childClass = cppFile.addClass("ChildClass");
                childClass.addParentClass(parentClass);

                const parentClasses = childClass.getParentClasses();

                assert.strictEqual(parentClasses.length, 1);
                assertCppClassEquals(parentClasses[0], parentClass);
            });
        });
    });

    test("Reopen database before comparison", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "parent_cpp_class_chain_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "parent_cpp_class_chain.json"
                );
                const grandParentClass = cppFile.addClass("GrandParentClass");
                const parentClass = cppFile.addClass("ParentClass");
                parentClass.addParentClass(grandParentClass);
                const childClass = cppFile.addClass("ChildClass");
                childClass.addParentClass(parentClass);

                database.writeDatabase();

                const newDatabase = openDatabase(__dirname, DatabaseType.sqlite);

                assertDatabaseEquals(newDatabase, referenceDatabase);
            });
        });
    });

    test("equality with simple parent class in different files", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "parent_cpp_class_in_hpp_expected_db.json",
                        DatabaseType.sqlite
                    );
                const hppFile = database.getOrAddHppFile("ParentClass.h");
                const parentClass = hppFile.addClass("ParentClass");
                const cppFile = database.getOrAddCppFile("ChildClass.cpp");
                const childClass = cppFile.addClass("ChildClass");

                childClass.addParentClass(parentClass);

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    test("Removed all database content", () => {

                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_parent_cpp_class_expected_db.json",
                        DatabaseType.sqlite
                    );
                const cppFile = database.getOrAddCppFile(
                    "simple_parent_cpp_class.json"
                );
                const parentClass = cppFile.addClass("ParentClass");
                const childClass = cppFile.addClass("ChildClass");

                childClass.addParentClass(parentClass);

                database.writeDatabase();

                assertDatabaseEquals(database, referenceDatabase);

                database.removeCppFileAndDependingContent(cppFile.getName());
                database.writeDatabase();
                assertDatabaseEquals(database, getEmptyReferenceDatabase());
            });
        });
    });
});
