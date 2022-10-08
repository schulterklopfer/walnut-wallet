/*
import 'package:path/path.dart';
import '../models/mint.dart';
import 'package:sqflite/sqflite.dart';

class DatabaseProvider {
  static const _databaseFile = 'data.sqlite';
  static final DatabaseProvider _instance = DatabaseProvider._init();
  static Database? _db;

  DatabaseProvider._init();

  Future<Database> get db async {
    if (_db != null) return _db!;
    _db = await _useDatabase(_databaseFile);
    return _db!;
  }

  Future<Database> _useDatabase(String filePath) async {
    final dbPath = await getDatabasesPath();
    return await openDatabase(
      join(dbPath, _databaseFile),
      onCreate: (db, version) {
        return db.execute(
            'CREATE TABLE mints (id INTEGER PRIMARY KEY, title TEXT, content TEXT)');
      },
      version: 1,
    );
  }

  Future<List<Mint>> findMints() async {
    final db = await _instance.db;
    final result = await db.rawQuery('SELECT * FROM mints ORDER BY id');
    // print(result);
    return result.map((json) => Mint.fromJson(json)).toList();
  }

  Future<Mint> insertMint(Mint mint) async {
    // TODO: what happens on conflict?
    final db = await _instance.db;
    final id = await db.rawInsert(
        'INSERT INTO mints (title, content) VALUES (?,?)',
        [mint.title, mint.content]);
    return mint.copy(id: id);
  }

  Future<Mint> updateMint(Mint mint) async {
    final db = await _instance.db;
    final id = await db.rawUpdate(
        'UPDATE mints SET title = ?, content = ? WHERE id = ?',
        [mint.title, mint.content, mint.id]);
    return mint.copy(id: id);
  }

  Future<int> deleteAllMints() async {
    final db = await _instance.db;
    final result = await db.rawDelete('DELETE FROM mints');
    return result;
  }

  Future<int> deleteMint(int mintId) async {
    final db = await _instance.db;
    final id = await db.rawDelete('DELETE FROM mints WHERE id = ?', [mintId]);
    return id;
  }

  Future close() async {
    final db = await _instance.db;
    db.close();
  }
}
*/