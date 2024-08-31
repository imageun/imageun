#!/bin/env fish
ast-grep run --pattern 'Luma([$M1])' --rewrite 'Luma{ v: $M1 }' --lang rust -U
ast-grep run --pattern 'LumaA([$M1, $M2])' --rewrite 'LumaA{ v: $M1, a: $M2 }' --lang rust -U
ast-grep run --pattern 'Rgb([$M1, $M2, $M3])' --rewrite 'Rgb{ r: $M1, g: $M2, b: $M3 }' --lang rust -U
ast-grep run --pattern 'Rgba([$M1, $M2, $M3, $M4])' --rewrite 'Rgba{ r: $M1, g: $M2, b: $M3, a: $M4 }' --lang rust -U

ast-grep run --pattern 'Luma($M1)' --rewrite 'Luma::from($M1)' --lang rust -U
ast-grep run --pattern 'LumaA($M1)' --rewrite 'LumaA::from($M1)' --lang rust -U
ast-grep run --pattern 'Rgb($M1)' --rewrite 'Rgb::from($M1)' --lang rust -U
ast-grep run --pattern 'Rgba($M1)' --rewrite 'Rgba::from($M1)' --lang rust -U

ast-grep run --pattern '<$M1 as Pixel>::from_slice($M2)' --rewrite 'bytemuck::cast_ref::<_, $M1>($M2)' --lang rust -U
ast-grep run --pattern '<$M1 as Pixel>::from_slice_mut($M2)' --rewrite 'bytemuck::cast_mut::<_, $M1>($M2)' --lang rust -U

ast-grep run --pattern '$M1::from_slice($M2)' --rewrite 'bytemuck::cast_ref::<_, $M1>($M2)' --lang rust -U
ast-grep run --pattern '$M1::from_slice_mut($M2)' --rewrite 'bytemuck::cast_mut::<_, $M1>($M2)' --lang rust -U

ast-grep run --pattern '$M1.channels()' --rewrite '$M1.as_array()' --lang rust -U
ast-grep run --pattern '$M1.channels_mut()' --rewrite '$M1.as_array_mut()' --lang rust -U

sd -F 'Subpixel' 'Component' src/**.rs benches/**.rs examples/**.rs tests/**.rs
sd -F '<P as Pixel>::CHANNEL_COUNT' '<P as rgb::HetPixel>::NUM_COMPONENTS' src/**.rs benches/**.rs examples/**.rs tests/**.rs
sd -F 'P::CHANNEL_COUNT' '<P as rgb::HetPixel>::NUM_COMPONENTS' src/**.rs benches/**.rs examples/**.rs tests/**.rs
