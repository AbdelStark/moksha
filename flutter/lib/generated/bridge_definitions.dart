// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.77.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'bridge_generated.io.dart'
    if (dart.library.html) 'bridge_generated.web.dart';
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

abstract class Native {
  Future<void> initCashu({required String dbPath, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kInitCashuConstMeta;

  Future<int> getBalance({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetBalanceConstMeta;

  Future<int> mintTokens(
      {required int amount, required String hash, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kMintTokensConstMeta;

  Future<FlutterPaymentRequest> getMintPaymentRequest(
      {required int amount, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetMintPaymentRequestConstMeta;

  Future<bool> payInvoice({required String invoice, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kPayInvoiceConstMeta;

  Future<int> importToken({required String token, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kImportTokenConstMeta;
}

class FlutterPaymentRequest {
  final String pr;
  final String hash;

  const FlutterPaymentRequest({
    required this.pr,
    required this.hash,
  });
}
