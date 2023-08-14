import 'dart:async';

import 'package:isolate/isolate.dart';

abstract class ISOManager {
  //提供外部首次初始化前修改
  static int isoBalanceSize = 2;

  //LoadBalancer 2个单位的线程池
  static Future<LoadBalancer> _loadBalancer =
      LoadBalancer.create(isoBalanceSize, IsolateRunner.spawn);

  //通过iso在新的线程中执行future内容体
  //R 为Future返回泛型，P 为方法入参泛型
  //function 必须为 static 方法
  static Future<R> loadBalanceFuture<R, P>(
    FutureOr<R> Function(P argument) function,
    P params,
  ) async {
    final lb = await _loadBalancer;
    return lb.run<R, P>(function, params);
  }
}

