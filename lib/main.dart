import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'color_schemes.g.dart';
import 'router.dart';

// https://riverpod.dev/
// https://github.com/anthonyaquino83/riverpodsqlitecrud/blob/main/lib/views/note_list.dart
// https://github.com/lucavenir/go_router_riverpod
// https://codelabs.developers.google.com/codelabs/mdc-103-flutter#9
// https://codelabs.developers.google.com/codelabs/flutter-boring-to-beautiful?hl=en#0
// https://docs.flutter.dev/development/ui/widgets
// https://m3.material.io/theme-builder#/dynamic
// https://bestflutterpack.com/
// https://bestflutterpack.com/how-to-showcase-features-in-flutter/
// https://bestflutterpack.com/flutter-package-providing-avatar-glow-widget/
// https://bestflutterpack.com/floating-navigation-bar-advanced-custom-extended-scaffold/
// https://bestflutterpack.com/basic-input-formatter-package-for-flutter/
// https://bestflutterpack.com/customisable-timelines-in-flutter/
// https://bestflutterpack.com/convert-your-flutter-button-into-an-animated-loading-button/
// https://bestflutterpack.com/progress-bar-widgets-for-flutter/
// https://bestflutterpack.com/how-to-build-pull-down-to-refresh-functionality-in-flutter/
// https://bestflutterpack.com/infinite-scroll-pagination-in-flutter/
void main() {
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends ConsumerWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider); // Try the latest async redirect!

    return MaterialApp.router(
      theme: ThemeData(primarySwatch: Colors.purple),
      //darkTheme: ThemeData(useMaterial3: true, colorScheme: darkColorScheme),
      debugShowCheckedModeBanner: false,
      routeInformationParser: router.routeInformationParser,
      routerDelegate: router.routerDelegate,
      routeInformationProvider: router.routeInformationProvider,
      title: 'flutter_riverpod + go_router Demo',
    );
  }
}
