#[no_mangle]
pub fn main() {
    let fn_get_2d_context = ramen::register_function(
        r#"
            (context, selectorStart, selectorEnd) => {
                let selector = context.getUtf8FromMemory(selectorStart,selectorEnd);
                let obj = document.querySelector(selector);
                return context.storeObject(obj.getContext("2d"));
            }"#,
    );
    let fn_set_color = ramen::register_function(
        r#"
            (context, ctxHandle, colorStart, colorEnd) => {
                let color = context.getUtf8FromMemory(colorStart, colorEnd);
                let ctx = context.getObject(ctxHandle);
                ctx.fillStyle = color;
            }"#,
    );
    let fn_fill_rect = ramen::register_function(
        r#"
            (context, ctxHandle, x, y, width, height) => {
                let ctx = context.getObject(ctxHandle);
                ctx.fillRect(x, y, width, height);
            }"#,
    );
    let target = "#screen";
    let ctx = fn_get_2d_context.invoke_2(target.as_ptr() as u32,target.len() as u32);
    let color = "red";
    fn_set_color.invoke_3(ctx, color.as_ptr() as u32, color.len() as u32);
    fn_fill_rect.invoke_5(ctx, 10.0, 10.0, 50.0, 50.0);
    let color = "blue";
    fn_set_color.invoke_3(ctx, color.as_ptr() as u32, color.len() as u32);
    fn_fill_rect.invoke_5(ctx, 20.0, 20.0, 50.0, 50.0);
    let color = "green";
    fn_set_color.invoke_3(ctx, color.as_ptr() as u32, color.len() as u32);
    fn_fill_rect.invoke_5(ctx, 30.0, 30.0, 50.0, 50.0);
}