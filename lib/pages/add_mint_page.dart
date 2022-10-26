import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:walnut/providers/mints.dart';

import '../ffi.dart';

/* "{\"members\":[[0,\"wss://fm-signet.sirion.io\"]]}" */
class AddMintPage extends ConsumerWidget {
  const AddMintPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    //Size screenSize = MediaQuery.of(context).size;
    final ClientListNotifier clientListNotifier =
        ref.watch(clientListProvider.notifier);

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
          child: TextButton(
        style: ButtonStyle(
          foregroundColor: MaterialStateProperty.all<Color>(Colors.blue),
        ),
        onPressed: () async {
          try {
            await clientListNotifier.joinFederation(
                '{"members":[[0,"wss://fm-signet.sirion.io"]]}');
            context.pop();
          } catch (e) {
            ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                behavior: SnackBarBehavior.fixed,
                content: Container(
                  padding: const EdgeInsets.all(8),
                  height: 70,
                  decoration: const BoxDecoration(
                    color: Colors.blue,
                    borderRadius: BorderRadius.all(Radius.circular(15)),
                  ),
                  child: Center(
                    child: Text(
                      e.toString(),
                    ),
                  ),
                )));
          }
        },
        child: Text('TextButton'),
      )),
    );
  }
}
