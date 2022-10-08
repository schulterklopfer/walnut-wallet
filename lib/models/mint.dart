// To parse this JSON data, do
//
//     final note = noteFromJson(jsonString);

import 'dart:convert';

List<Mint> noteFromJson(String str) =>
    List<Mint>.from(json.decode(str).map((x) => Mint.fromJson(x)));

String noteToJson(List<Mint> data) =>
    json.encode(List<dynamic>.from(data.map((x) => x.toJson())));

class Mint {
  Mint({
    this.id,
    required this.title,
    required this.content,
  });

  int? id;
  String title;
  String content;

  factory Mint.fromJson(Map<String, dynamic> json) => Mint(
        id: json["id"],
        title: json["title"],
        content: json["content"],
      );

  Map<String, dynamic> toJson() => {
        "id": id,
        "title": title,
        "content": content,
      };

  Mint copy({
    int? id,
    String? title,
    String? content,
  }) =>
      Mint(
        id: id ?? this.id,
        title: title ?? this.title,
        content: content ?? this.content,
      );
}
