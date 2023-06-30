import 'package:moksha_wallet/main.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class SettingsPage extends ConsumerWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    AsyncValue<String> config = ref.watch(dbPathProvider);

    var dbPath = config.when(
      loading: () => const CircularProgressIndicator(),
      error: (err, stack) => Text('Error: $err'),
      data: (db) {
        return db;
      },
    );

    return Scaffold(
        appBar: AppBar(
          title: const Text('Settings'),
          backgroundColor: Colors.black,
        ),
        body: Container(
          margin: const EdgeInsets.all(24),
          child: Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                const Text(
                  'Settings',
                ),
                const TextField(
                  decoration: InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: 'Mint URL',
                  ),
                ),
                Text(
                  'DB Path: $dbPath',
                  style: Theme.of(context).textTheme.titleSmall,
                )
              ],
            ),
          ),
        ));
  }
}
