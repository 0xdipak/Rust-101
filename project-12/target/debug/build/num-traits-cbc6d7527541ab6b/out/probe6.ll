; ModuleID = 'probe6.ed14347e0b178c8c-cgu.0'
source_filename = "probe6.ed14347e0b178c8c-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

; core::f64::<impl f64>::is_subnormal
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h423c459ad8c1d445E"(double %self) unnamed_addr #0 {
start:
  %_2 = alloca i8, align 1
; call core::f64::<impl f64>::classify
  %0 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17hd9e369ef4dd7b199E"(double %self), !range !1
  store i8 %0, ptr %_2, align 1
  %1 = load i8, ptr %_2, align 1, !range !1, !noundef !2
  %_3 = zext i8 %1 to i64
  %2 = icmp eq i64 %_3, 3
  ret i1 %2
}

; probe6::probe
; Function Attrs: uwtable
define void @_ZN6probe65probe17hf6b8bdb0a13d24e6E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::is_subnormal
  %_1 = call zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h423c459ad8c1d445E"(double 1.000000e+00)
  ret void
}

; core::f64::<impl f64>::classify
; Function Attrs: uwtable
declare i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17hd9e369ef4dd7b199E"(double) unnamed_addr #1

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i8 0, i8 5}
!2 = !{}
