import 'package:card_swiper/card_swiper.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:walnut/constants.dart';

const _wallets = ["fedimint 1", "fedimint 2", "cashu 1"];

class RootPage extends ConsumerWidget {
  RootPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    Size screenSize = MediaQuery.of(context).size;
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
              child: Swiper(
                layout: SwiperLayout.DEFAULT,
                viewportFraction: 0.85,
                itemBuilder: (BuildContext context, int index) {
                  return Card(
                      margin: EdgeInsets.fromLTRB(
                          5, 10, 5, _wallets.length > 1 ? 35 : 10),
                      child: Center(child: Text(_wallets[index])));
                },
                itemCount: _wallets.length,
                loop: false,
                pagination: _wallets.length > 1
                    ? const SwiperPagination(
                        alignment: Alignment.bottomCenter,
                        builder: DotSwiperPaginationBuilder(
                          color: Colors.grey,
                        ),
                      )
                    : null,
              ),
            ),
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
