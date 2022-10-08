import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

class AddMintPage extends ConsumerWidget {
  const AddMintPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    //Size screenSize = MediaQuery.of(context).size;
    return Scaffold(
      appBar: AppBar(
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.only(
              bottomRight: Radius.circular(25),
              bottomLeft: Radius.circular(25)),
        ),
        automaticallyImplyLeading: false,
        actions: [
          IconButton(
              onPressed: () {
                context.pop();
              },
              icon: const Icon(Icons.close))
        ],
        title: const Text('Add mint'),
      ),
      body: Center(
        child: Text('add mint page'),
      ),
    );
  }
}
