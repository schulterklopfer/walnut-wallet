import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../ffi.dart';

final clientListNotifier = ClientListNotifier();

final clientListProvider = StateNotifierProvider<ClientListNotifier,
    AsyncValue<List<BridgeClientInfo>>>((ref) => ClientListNotifier());

class ClientListNotifier
    extends StateNotifier<AsyncValue<List<BridgeClientInfo>>> {
  ClientListNotifier() : super(const AsyncLoading()) {
    getClients();
  }

  void getClients() async {
    state = await AsyncValue.guard(() async {
      return await api.getClients();
    });
  }

  Future joinFederation(configUrl) async {
    await api.joinFederation(configUrl: configUrl);
    getClients();
  }

  Future leaveFederation(label) async {
    await api.leaveFederation(label: label);
    getClients();
  }
}
