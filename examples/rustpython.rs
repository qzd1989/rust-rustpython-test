use rustpython_vm as vm;
fn main() -> vm::PyResult<()> {
    let vm = vm::Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    });
    vm.enter(|vm| {
        vm.insert_sys_path(vm.new_pyobj("python_project"))?;
        let scope = vm.new_scope_with_builtins();
        let source_path = "python_project/main.py";
        let source = std::fs::read_to_string(source_path).expect("unable to read main.py");
        let code_obj = vm
            .compile(&source, vm::compiler::Mode::Exec, source_path.into())
            .map_err(|err| vm.new_syntax_error(&err, Some(&source)))?;
        if let Err(err) = vm.run_code_obj(code_obj, scope) {
            vm.print_exception(err);
        }
        Ok(())
    })
}
