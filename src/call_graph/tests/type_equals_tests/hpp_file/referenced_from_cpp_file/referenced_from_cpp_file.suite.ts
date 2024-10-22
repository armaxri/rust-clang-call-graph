import assert from "assert";
import { DatabaseType } from "../../../../../../backend/Config";
import { addSuitesInSubDirsSuites } from "../../../../helper/mocha_test_helper";
import { prepareDatabaseEqualityTests } from "../../database_equality_tests";
import { assertDatabaseEquals } from "../../../../helper/database_equality";

suite("Referenced from Cpp Files", () => {
    addSuitesInSubDirsSuites(__dirname);

    suite("Equality with simple parent class", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_referenced_from_cpp_file_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile("base.cpp");
                const hppFile = database.getOrAddHppFile("base.hpp");

                hppFile.addReferencedFromFile(cppFile.getName());

                database.writeDatabase();

                assert.strictEqual(hppFile.getReferencedFromFiles().length, 1);
                assert.strictEqual(
                    hppFile.getReferencedFromFiles()[0],
                    cppFile.getName()
                );

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    suite("Get or add with simple parent class", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_referenced_from_cpp_file_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile("base.cpp");
                const hppFile = database.getOrAddHppFile("base.hpp");

                hppFile.addReferencedFromFile(cppFile.getName());
                hppFile.addReferencedFromFile(cppFile.getName());

                database.writeDatabase();

                assert.strictEqual(hppFile.getReferencedFromFiles().length, 1);
                assert.strictEqual(
                    hppFile.getReferencedFromFiles()[0],
                    cppFile.getName()
                );

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });

    suite("No equality with simple parent class (missing connection)", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_referenced_from_cpp_file_expected_db.json",
                        testData
                    );
                database.getOrAddCppFile("base.cpp");
                const hppFile = database.getOrAddHppFile("base.hpp");

                database.writeDatabase();

                assert.strictEqual(hppFile.getReferencedFromFiles().length, 0);

                assert.throws(() =>
                    assertDatabaseEquals(database, referenceDatabase)
                );
            });
        });
    });

    suite("Equality with simple header referenced by a header", () => {
        [DatabaseType.lowdb, DatabaseType.sqlite].forEach((testData) => {
            test(`${DatabaseType[testData]}`, () => {
                const [database, referenceDatabase] =
                    prepareDatabaseEqualityTests(
                        __dirname,
                        "simple_referenced_from_hpp_file_expected_db.json",
                        testData
                    );
                const cppFile = database.getOrAddCppFile("main.cpp");
                const hppFile1 = database.getOrAddHppFile("firstHeader.hpp");
                const hppFile2 = database.getOrAddHppFile("secondHeader.hpp");

                hppFile1.addReferencedFromFile(cppFile.getName());
                hppFile1.addReferencedFromFile(hppFile2.getName());
                hppFile2.addReferencedFromFile(cppFile.getName());

                database.writeDatabase();

                assert.strictEqual(hppFile1.getReferencedFromFiles().length, 2);
                assert.deepStrictEqual(hppFile1.getReferencedFromFiles(), [
                    cppFile.getName(),
                    hppFile2.getName(),
                ]);

                assert.strictEqual(hppFile2.getReferencedFromFiles().length, 1);
                assert.strictEqual(
                    hppFile2.getReferencedFromFiles()[0],
                    cppFile.getName()
                );

                assertDatabaseEquals(database, referenceDatabase);
            });
        });
    });
});
