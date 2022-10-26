import 'package:card_swiper/card_swiper.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:walnut/constants.dart';

import '../ffi.dart';
import '../providers/mints.dart';

class RootPage extends ConsumerWidget {
  const RootPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    Size screenSize = MediaQuery.of(context).size;

    final clients = ref.watch(clientListProvider);
    final ClientListNotifier clientListNotifier =
        ref.watch(clientListProvider.notifier);
    return Scaffold(
        appBar: AppBar(
          title: const Text('Wallets'),
        ),
        body: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            SizedBox(
                width: screenSize.width,
                height: screenSize.height * 0.3,
                child: clients.when(
                  data: (l) => Swiper(
                    layout: SwiperLayout.DEFAULT,
                    viewportFraction: 0.85,
                    itemBuilder: (BuildContext context, int index) {
                      return Card(
                          margin: EdgeInsets.fromLTRB(
                              5, 10, 5, l.length > 1 ? 35 : 10),
                          child: Column(
                            children: [
                              Center(child: Text(l[index].label)),
                              TextButton(
                                style: ButtonStyle(
                                  foregroundColor:
                                      MaterialStateProperty.all<Color>(
                                          Colors.blue),
                                ),
                                onPressed: () async {
                                  try {
                                    clientListNotifier
                                        .leaveFederation(l[index].label);
                                  } catch (e) {
                                    ScaffoldMessenger.of(context)
                                        .showSnackBar(SnackBar(
                                            behavior: SnackBarBehavior.fixed,
                                            content: Container(
                                              padding: const EdgeInsets.all(8),
                                              height: 70,
                                              decoration: const BoxDecoration(
                                                color: Colors.blue,
                                                borderRadius: BorderRadius.all(
                                                    Radius.circular(15)),
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
                              )
                            ],
                          ));
                    },
                    itemCount: l.length,
                    loop: false,
                    pagination: l.length > 1
                        ? const SwiperPagination(
                            alignment: Alignment.bottomCenter,
                            builder: DotSwiperPaginationBuilder(
                              color: Colors.grey,
                            ),
                          )
                        : null,
                  ),
                  loading: () =>
                      const Center(child: CircularProgressIndicator()),
                  error: (e, st) => Center(child: Text(e.toString())),
                )),
          ],
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: () {
            context.pushNamed(addMintRouteName);
          },
          mini: false,
          shape: CircleBorder(side: BorderSide.none),
          child: Icon(Icons.add),
        ));
  }
}
