// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.48.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names

import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'package:meta/meta.dart';
import 'package:meta/meta.dart';
import 'dart:ffi' as ffi;

abstract class MinimintBridge {
  Future<List<BridgeClientInfo>> init({required String path, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kInitConstMeta;

  Future<void> deleteDatabase({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kDeleteDatabaseConstMeta;

  Future<void> saveAppData({required String data, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSaveAppDataConstMeta;

  Future<String?> fetchAppData({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kFetchAppDataConstMeta;

  Future<String?> removeAppData({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kRemoveAppDataConstMeta;

  Future<BridgeClientInfo> getClient({required String label, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetClientConstMeta;

  Future<List<BridgeClientInfo>> getClients({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetClientsConstMeta;

  Future<BridgeClientInfo> joinFederation(
      {required String configUrl, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kJoinFederationConstMeta;

  /// Unset client and wipe database. Ecash will be destroyed. Use with caution!!!
  Future<void> leaveFederation({required String label, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kLeaveFederationConstMeta;

  Future<int> balance({required String label, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kBalanceConstMeta;

  Future<void> pay(
      {required String label, required String bolt11, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kPayConstMeta;

  Future<String> invoice(
      {required String label,
      required int amount,
      required String description,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kInvoiceConstMeta;

  Future<BridgePayment> fetchPayment(
      {required String label, required String paymentHash, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kFetchPaymentConstMeta;

  Future<List<BridgePayment>> listPayments(
      {required String label, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kListPaymentsConstMeta;

  Future<bool> configuredStatus({required String label, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConfiguredStatusConstMeta;

  Future<ConnectionStatus> connectionStatus(
      {required String label, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConnectionStatusConstMeta;

  Future<String> network({required String label, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kNetworkConstMeta;

  Future<int?> calculateFee({required String bolt11, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kCalculateFeeConstMeta;

  /// Returns the federations we're members of
  ///
  /// At most one will be `active`
  Future<List<BridgeFederationInfo>> listFederations({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kListFederationsConstMeta;

  /// Switch to a federation that we've already joined
  ///
  /// This assumes federation config is already saved locally
  Future<void> switchFederation(
      {required BridgeFederationInfo federation, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSwitchFederationConstMeta;

  /// Decodes an invoice and checks that we can pay it
  Future<BridgeInvoice> decodeInvoice(
      {required String label, required String bolt11, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kDecodeInvoiceConstMeta;
}

/// Bridge representation of a fedimint node
class BridgeClientInfo {
  final String label;
  final int balance;
  final String configJson;
  final ConnectionStatus connectionStatus;
  final String? federationName;
  final String? userData;

  BridgeClientInfo({
    required this.label,
    required this.balance,
    required this.configJson,
    required this.connectionStatus,
    this.federationName,
    this.userData,
  });
}

/// Bridge representation of a fedimint node
class BridgeFederationInfo {
  final String name;
  final String network;
  final bool current;
  final List<BridgeGuardianInfo> guardians;

  BridgeFederationInfo({
    required this.name,
    required this.network,
    required this.current,
    required this.guardians,
  });
}

/// Bridge representation of a fedimint node
class BridgeGuardianInfo {
  final String name;
  final String address;
  final bool online;

  BridgeGuardianInfo({
    required this.name,
    required this.address,
    required this.online,
  });
}

class BridgeInvoice {
  final String paymentHash;
  final int amount;
  final String description;
  final String invoice;

  BridgeInvoice({
    required this.paymentHash,
    required this.amount,
    required this.description,
    required this.invoice,
  });
}

class BridgePayment {
  final BridgeInvoice invoice;
  final PaymentStatus status;
  final int createdAt;
  final bool paid;
  final PaymentDirection direction;

  BridgePayment({
    required this.invoice,
    required this.status,
    required this.createdAt,
    required this.paid,
    required this.direction,
  });
}

enum ConnectionStatus {
  NotConnected,
  Connected,
  NotConfigured,
}

enum PaymentDirection {
  Outgoing,
  Incoming,
}

enum PaymentStatus {
  Paid,
  Pending,
  Failed,
  Expired,
}

class MinimintBridgeImpl implements MinimintBridge {
  final MinimintBridgePlatform _platform;
  factory MinimintBridgeImpl(ExternalLibrary dylib) =>
      MinimintBridgeImpl.raw(MinimintBridgePlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory MinimintBridgeImpl.wasm(FutureOr<WasmModule> module) =>
      MinimintBridgeImpl(module as ExternalLibrary);
  MinimintBridgeImpl.raw(this._platform);
  Future<List<BridgeClientInfo>> init({required String path, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) =>
            _platform.inner.wire_init(port_, _platform.api2wire_String(path)),
        parseSuccessData: _wire2api_list_bridge_client_info,
        constMeta: kInitConstMeta,
        argValues: [path],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kInitConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "init",
        argNames: ["path"],
      );

  Future<void> deleteDatabase({dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_delete_database(port_),
        parseSuccessData: _wire2api_unit,
        constMeta: kDeleteDatabaseConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kDeleteDatabaseConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "delete_database",
        argNames: [],
      );

  Future<void> saveAppData({required String data, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_save_app_data(port_, _platform.api2wire_String(data)),
        parseSuccessData: _wire2api_unit,
        constMeta: kSaveAppDataConstMeta,
        argValues: [data],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kSaveAppDataConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "save_app_data",
        argNames: ["data"],
      );

  Future<String?> fetchAppData({dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_fetch_app_data(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: kFetchAppDataConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kFetchAppDataConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "fetch_app_data",
        argNames: [],
      );

  Future<String?> removeAppData({dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_remove_app_data(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: kRemoveAppDataConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kRemoveAppDataConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "remove_app_data",
        argNames: [],
      );

  Future<BridgeClientInfo> getClient({required String label, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_get_client(port_, _platform.api2wire_String(label)),
        parseSuccessData: _wire2api_bridge_client_info,
        constMeta: kGetClientConstMeta,
        argValues: [label],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kGetClientConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_client",
        argNames: ["label"],
      );

  Future<List<BridgeClientInfo>> getClients({dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_get_clients(port_),
        parseSuccessData: _wire2api_list_bridge_client_info,
        constMeta: kGetClientsConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kGetClientsConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "get_clients",
        argNames: [],
      );

  Future<BridgeClientInfo> joinFederation(
          {required String configUrl, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_join_federation(port_, _platform.api2wire_String(configUrl)),
        parseSuccessData: _wire2api_bridge_client_info,
        constMeta: kJoinFederationConstMeta,
        argValues: [configUrl],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kJoinFederationConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "join_federation",
        argNames: ["configUrl"],
      );

  Future<void> leaveFederation({required String label, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_leave_federation(port_, _platform.api2wire_String(label)),
        parseSuccessData: _wire2api_unit,
        constMeta: kLeaveFederationConstMeta,
        argValues: [label],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kLeaveFederationConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "leave_federation",
        argNames: ["label"],
      );

  Future<int> balance({required String label, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_balance(port_, _platform.api2wire_String(label)),
        parseSuccessData: _wire2api_u64,
        constMeta: kBalanceConstMeta,
        argValues: [label],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kBalanceConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "balance",
        argNames: ["label"],
      );

  Future<void> pay(
          {required String label, required String bolt11, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_pay(
            port_,
            _platform.api2wire_String(label),
            _platform.api2wire_String(bolt11)),
        parseSuccessData: _wire2api_unit,
        constMeta: kPayConstMeta,
        argValues: [label, bolt11],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kPayConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "pay",
        argNames: ["label", "bolt11"],
      );

  Future<String> invoice(
          {required String label,
          required int amount,
          required String description,
          dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_invoice(
            port_,
            _platform.api2wire_String(label),
            _platform.api2wire_u64(amount),
            _platform.api2wire_String(description)),
        parseSuccessData: _wire2api_String,
        constMeta: kInvoiceConstMeta,
        argValues: [label, amount, description],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kInvoiceConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "invoice",
        argNames: ["label", "amount", "description"],
      );

  Future<BridgePayment> fetchPayment(
          {required String label, required String paymentHash, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_fetch_payment(
            port_,
            _platform.api2wire_String(label),
            _platform.api2wire_String(paymentHash)),
        parseSuccessData: _wire2api_bridge_payment,
        constMeta: kFetchPaymentConstMeta,
        argValues: [label, paymentHash],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kFetchPaymentConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "fetch_payment",
        argNames: ["label", "paymentHash"],
      );

  Future<List<BridgePayment>> listPayments(
          {required String label, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_list_payments(port_, _platform.api2wire_String(label)),
        parseSuccessData: _wire2api_list_bridge_payment,
        constMeta: kListPaymentsConstMeta,
        argValues: [label],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kListPaymentsConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "list_payments",
        argNames: ["label"],
      );

  Future<bool> configuredStatus({required String label, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_configured_status(port_, _platform.api2wire_String(label)),
        parseSuccessData: _wire2api_bool,
        constMeta: kConfiguredStatusConstMeta,
        argValues: [label],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConfiguredStatusConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "configured_status",
        argNames: ["label"],
      );

  Future<ConnectionStatus> connectionStatus(
          {required String label, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_connection_status(port_, _platform.api2wire_String(label)),
        parseSuccessData: _wire2api_connection_status,
        constMeta: kConnectionStatusConstMeta,
        argValues: [label],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConnectionStatusConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "connection_status",
        argNames: ["label"],
      );

  Future<String> network({required String label, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_network(port_, _platform.api2wire_String(label)),
        parseSuccessData: _wire2api_String,
        constMeta: kNetworkConstMeta,
        argValues: [label],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kNetworkConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "network",
        argNames: ["label"],
      );

  Future<int?> calculateFee({required String bolt11, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner
            .wire_calculate_fee(port_, _platform.api2wire_String(bolt11)),
        parseSuccessData: _wire2api_opt_box_autoadd_u64,
        constMeta: kCalculateFeeConstMeta,
        argValues: [bolt11],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kCalculateFeeConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "calculate_fee",
        argNames: ["bolt11"],
      );

  Future<List<BridgeFederationInfo>> listFederations({dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_list_federations(port_),
        parseSuccessData: _wire2api_list_bridge_federation_info,
        constMeta: kListFederationsConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kListFederationsConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "list_federations",
        argNames: [],
      );

  Future<void> switchFederation(
          {required BridgeFederationInfo federation, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_switch_federation(port_,
            _platform.api2wire_box_autoadd_bridge_federation_info(federation)),
        parseSuccessData: _wire2api_unit,
        constMeta: kSwitchFederationConstMeta,
        argValues: [federation],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kSwitchFederationConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "switch_federation",
        argNames: ["federation"],
      );

  Future<BridgeInvoice> decodeInvoice(
          {required String label, required String bolt11, dynamic hint}) =>
      _platform.executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => _platform.inner.wire_decode_invoice(
            port_,
            _platform.api2wire_String(label),
            _platform.api2wire_String(bolt11)),
        parseSuccessData: _wire2api_bridge_invoice,
        constMeta: kDecodeInvoiceConstMeta,
        argValues: [label, bolt11],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kDecodeInvoiceConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "decode_invoice",
        argNames: ["label", "bolt11"],
      );
}

// Section: api2wire

@protected
bool api2wire_bool(bool raw) {
  return raw;
}

@protected
int api2wire_u8(int raw) {
  return raw;
}

// Section: wire2api

String _wire2api_String(dynamic raw) {
  return raw as String;
}

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
}

int _wire2api_box_autoadd_u64(dynamic raw) {
  return _wire2api_u64(raw);
}

BridgeClientInfo _wire2api_bridge_client_info(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 6)
    throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
  return BridgeClientInfo(
    label: _wire2api_String(arr[0]),
    balance: _wire2api_u64(arr[1]),
    configJson: _wire2api_String(arr[2]),
    connectionStatus: _wire2api_connection_status(arr[3]),
    federationName: _wire2api_opt_String(arr[4]),
    userData: _wire2api_opt_String(arr[5]),
  );
}

BridgeFederationInfo _wire2api_bridge_federation_info(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return BridgeFederationInfo(
    name: _wire2api_String(arr[0]),
    network: _wire2api_String(arr[1]),
    current: _wire2api_bool(arr[2]),
    guardians: _wire2api_list_bridge_guardian_info(arr[3]),
  );
}

BridgeGuardianInfo _wire2api_bridge_guardian_info(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 3)
    throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
  return BridgeGuardianInfo(
    name: _wire2api_String(arr[0]),
    address: _wire2api_String(arr[1]),
    online: _wire2api_bool(arr[2]),
  );
}

BridgeInvoice _wire2api_bridge_invoice(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return BridgeInvoice(
    paymentHash: _wire2api_String(arr[0]),
    amount: _wire2api_u64(arr[1]),
    description: _wire2api_String(arr[2]),
    invoice: _wire2api_String(arr[3]),
  );
}

BridgePayment _wire2api_bridge_payment(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 5)
    throw Exception('unexpected arr length: expect 5 but see ${arr.length}');
  return BridgePayment(
    invoice: _wire2api_bridge_invoice(arr[0]),
    status: _wire2api_payment_status(arr[1]),
    createdAt: _wire2api_u64(arr[2]),
    paid: _wire2api_bool(arr[3]),
    direction: _wire2api_payment_direction(arr[4]),
  );
}

ConnectionStatus _wire2api_connection_status(dynamic raw) {
  return ConnectionStatus.values[raw];
}

int _wire2api_i32(dynamic raw) {
  return raw as int;
}

List<BridgeClientInfo> _wire2api_list_bridge_client_info(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_bridge_client_info).toList();
}

List<BridgeFederationInfo> _wire2api_list_bridge_federation_info(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_bridge_federation_info).toList();
}

List<BridgeGuardianInfo> _wire2api_list_bridge_guardian_info(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_bridge_guardian_info).toList();
}

List<BridgePayment> _wire2api_list_bridge_payment(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_bridge_payment).toList();
}

String? _wire2api_opt_String(dynamic raw) {
  return raw == null ? null : _wire2api_String(raw);
}

int? _wire2api_opt_box_autoadd_u64(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_u64(raw);
}

PaymentDirection _wire2api_payment_direction(dynamic raw) {
  return PaymentDirection.values[raw];
}

PaymentStatus _wire2api_payment_status(dynamic raw) {
  return PaymentStatus.values[raw];
}

int _wire2api_u64(dynamic raw) {
  return castInt(raw);
}

int _wire2api_u8(dynamic raw) {
  return raw as int;
}

Uint8List _wire2api_uint_8_list(dynamic raw) {
  return raw as Uint8List;
}

void _wire2api_unit(dynamic raw) {
  return;
}

class MinimintBridgePlatform extends FlutterRustBridgeBase<MinimintBridgeWire> {
  MinimintBridgePlatform(ffi.DynamicLibrary dylib)
      : super(MinimintBridgeWire(dylib));
// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_BridgeFederationInfo>
      api2wire_box_autoadd_bridge_federation_info(BridgeFederationInfo raw) {
    final ptr = inner.new_box_autoadd_bridge_federation_info_0();
    _api_fill_to_wire_bridge_federation_info(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_list_bridge_guardian_info>
      api2wire_list_bridge_guardian_info(List<BridgeGuardianInfo> raw) {
    final ans = inner.new_list_bridge_guardian_info_0(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_bridge_guardian_info(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  int api2wire_u64(int raw) {
    return raw;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: api_fill_to_wire

  void _api_fill_to_wire_box_autoadd_bridge_federation_info(
      BridgeFederationInfo apiObj,
      ffi.Pointer<wire_BridgeFederationInfo> wireObj) {
    _api_fill_to_wire_bridge_federation_info(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_bridge_federation_info(
      BridgeFederationInfo apiObj, wire_BridgeFederationInfo wireObj) {
    wireObj.name = api2wire_String(apiObj.name);
    wireObj.network = api2wire_String(apiObj.network);
    wireObj.current = api2wire_bool(apiObj.current);
    wireObj.guardians = api2wire_list_bridge_guardian_info(apiObj.guardians);
  }

  void _api_fill_to_wire_bridge_guardian_info(
      BridgeGuardianInfo apiObj, wire_BridgeGuardianInfo wireObj) {
    wireObj.name = api2wire_String(apiObj.name);
    wireObj.address = api2wire_String(apiObj.address);
    wireObj.online = api2wire_bool(apiObj.online);
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class MinimintBridgeWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  MinimintBridgeWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  MinimintBridgeWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  void wire_init(
    int port_,
    ffi.Pointer<wire_uint_8_list> path,
  ) {
    return _wire_init(
      port_,
      path,
    );
  }

  late final _wire_initPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_init');
  late final _wire_init = _wire_initPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_delete_database(
    int port_,
  ) {
    return _wire_delete_database(
      port_,
    );
  }

  late final _wire_delete_databasePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_delete_database');
  late final _wire_delete_database =
      _wire_delete_databasePtr.asFunction<void Function(int)>();

  void wire_save_app_data(
    int port_,
    ffi.Pointer<wire_uint_8_list> data,
  ) {
    return _wire_save_app_data(
      port_,
      data,
    );
  }

  late final _wire_save_app_dataPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_save_app_data');
  late final _wire_save_app_data = _wire_save_app_dataPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_fetch_app_data(
    int port_,
  ) {
    return _wire_fetch_app_data(
      port_,
    );
  }

  late final _wire_fetch_app_dataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_fetch_app_data');
  late final _wire_fetch_app_data =
      _wire_fetch_app_dataPtr.asFunction<void Function(int)>();

  void wire_remove_app_data(
    int port_,
  ) {
    return _wire_remove_app_data(
      port_,
    );
  }

  late final _wire_remove_app_dataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_remove_app_data');
  late final _wire_remove_app_data =
      _wire_remove_app_dataPtr.asFunction<void Function(int)>();

  void wire_get_client(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
  ) {
    return _wire_get_client(
      port_,
      label,
    );
  }

  late final _wire_get_clientPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_get_client');
  late final _wire_get_client = _wire_get_clientPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_get_clients(
    int port_,
  ) {
    return _wire_get_clients(
      port_,
    );
  }

  late final _wire_get_clientsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_clients');
  late final _wire_get_clients =
      _wire_get_clientsPtr.asFunction<void Function(int)>();

  void wire_join_federation(
    int port_,
    ffi.Pointer<wire_uint_8_list> config_url,
  ) {
    return _wire_join_federation(
      port_,
      config_url,
    );
  }

  late final _wire_join_federationPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_join_federation');
  late final _wire_join_federation = _wire_join_federationPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_leave_federation(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
  ) {
    return _wire_leave_federation(
      port_,
      label,
    );
  }

  late final _wire_leave_federationPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_leave_federation');
  late final _wire_leave_federation = _wire_leave_federationPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_balance(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
  ) {
    return _wire_balance(
      port_,
      label,
    );
  }

  late final _wire_balancePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_balance');
  late final _wire_balance = _wire_balancePtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_pay(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
    ffi.Pointer<wire_uint_8_list> bolt11,
  ) {
    return _wire_pay(
      port_,
      label,
      bolt11,
    );
  }

  late final _wire_payPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_pay');
  late final _wire_pay = _wire_payPtr.asFunction<
      void Function(
          int, ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  void wire_invoice(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
    int amount,
    ffi.Pointer<wire_uint_8_list> description,
  ) {
    return _wire_invoice(
      port_,
      label,
      amount,
      description,
    );
  }

  late final _wire_invoicePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
              ffi.Uint64, ffi.Pointer<wire_uint_8_list>)>>('wire_invoice');
  late final _wire_invoice = _wire_invoicePtr.asFunction<
      void Function(int, ffi.Pointer<wire_uint_8_list>, int,
          ffi.Pointer<wire_uint_8_list>)>();

  void wire_fetch_payment(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
    ffi.Pointer<wire_uint_8_list> payment_hash,
  ) {
    return _wire_fetch_payment(
      port_,
      label,
      payment_hash,
    );
  }

  late final _wire_fetch_paymentPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_fetch_payment');
  late final _wire_fetch_payment = _wire_fetch_paymentPtr.asFunction<
      void Function(
          int, ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  void wire_list_payments(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
  ) {
    return _wire_list_payments(
      port_,
      label,
    );
  }

  late final _wire_list_paymentsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_list_payments');
  late final _wire_list_payments = _wire_list_paymentsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_configured_status(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
  ) {
    return _wire_configured_status(
      port_,
      label,
    );
  }

  late final _wire_configured_statusPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_configured_status');
  late final _wire_configured_status = _wire_configured_statusPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_connection_status(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
  ) {
    return _wire_connection_status(
      port_,
      label,
    );
  }

  late final _wire_connection_statusPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_connection_status');
  late final _wire_connection_status = _wire_connection_statusPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_network(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
  ) {
    return _wire_network(
      port_,
      label,
    );
  }

  late final _wire_networkPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_network');
  late final _wire_network = _wire_networkPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_calculate_fee(
    int port_,
    ffi.Pointer<wire_uint_8_list> bolt11,
  ) {
    return _wire_calculate_fee(
      port_,
      bolt11,
    );
  }

  late final _wire_calculate_feePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_calculate_fee');
  late final _wire_calculate_fee = _wire_calculate_feePtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_list_federations(
    int port_,
  ) {
    return _wire_list_federations(
      port_,
    );
  }

  late final _wire_list_federationsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_list_federations');
  late final _wire_list_federations =
      _wire_list_federationsPtr.asFunction<void Function(int)>();

  void wire_switch_federation(
    int port_,
    ffi.Pointer<wire_BridgeFederationInfo> _federation,
  ) {
    return _wire_switch_federation(
      port_,
      _federation,
    );
  }

  late final _wire_switch_federationPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_BridgeFederationInfo>)>>(
      'wire_switch_federation');
  late final _wire_switch_federation = _wire_switch_federationPtr
      .asFunction<void Function(int, ffi.Pointer<wire_BridgeFederationInfo>)>();

  void wire_decode_invoice(
    int port_,
    ffi.Pointer<wire_uint_8_list> label,
    ffi.Pointer<wire_uint_8_list> bolt11,
  ) {
    return _wire_decode_invoice(
      port_,
      label,
      bolt11,
    );
  }

  late final _wire_decode_invoicePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_decode_invoice');
  late final _wire_decode_invoice = _wire_decode_invoicePtr.asFunction<
      void Function(
          int, ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  ffi.Pointer<wire_BridgeFederationInfo>
      new_box_autoadd_bridge_federation_info_0() {
    return _new_box_autoadd_bridge_federation_info_0();
  }

  late final _new_box_autoadd_bridge_federation_info_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_BridgeFederationInfo>
              Function()>>('new_box_autoadd_bridge_federation_info_0');
  late final _new_box_autoadd_bridge_federation_info_0 =
      _new_box_autoadd_bridge_federation_info_0Ptr
          .asFunction<ffi.Pointer<wire_BridgeFederationInfo> Function()>();

  ffi.Pointer<wire_list_bridge_guardian_info> new_list_bridge_guardian_info_0(
    int len,
  ) {
    return _new_list_bridge_guardian_info_0(
      len,
    );
  }

  late final _new_list_bridge_guardian_info_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_bridge_guardian_info> Function(
              ffi.Int32)>>('new_list_bridge_guardian_info_0');
  late final _new_list_bridge_guardian_info_0 =
      _new_list_bridge_guardian_info_0Ptr.asFunction<
          ffi.Pointer<wire_list_bridge_guardian_info> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

class wire_BridgeGuardianInfo extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> name;

  external ffi.Pointer<wire_uint_8_list> address;

  @ffi.Bool()
  external bool online;
}

class wire_list_bridge_guardian_info extends ffi.Struct {
  external ffi.Pointer<wire_BridgeGuardianInfo> ptr;

  @ffi.Int32()
  external int len;
}

class wire_BridgeFederationInfo extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> name;

  external ffi.Pointer<wire_uint_8_list> network;

  @ffi.Bool()
  external bool current;

  external ffi.Pointer<wire_list_bridge_guardian_info> guardians;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
