MEMORY
{
  /* Карта памяти MIK32 для загрузки/выполнения из RAM */
  ROM (rx)  : ORIGIN = 0x02000000, LENGTH = 16K   /* Код размещаем в начале ОЗУ */
  RAM (rwx) : ORIGIN = 0x02004000, LENGTH = 16K   /* Данные и стек во второй половине ОЗУ */
}

/* Фиксы для совместимости с новыми версиями riscv-rt */
/*
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);
*/
/* Перенаправляем ранние прерывания на дефолтный аборт */
/*
PROVIDE(_pre_init_trap = _default_abort);
*/
/* Дополнительные заглушки для исключений и прерываний по умолчанию */
/*
PROVIDE(UserSoft = _default_handler);
PROVIDE(SupervisorSoft = _default_handler);
PROVIDE(MachineSoft = _default_handler);
PROVIDE(UserTimer = _default_handler);
PROVIDE(SupervisorTimer = _default_handler);
PROVIDE(MachineTimer = _default_handler);
PROVIDE(UserExternal = _default_handler);
PROVIDE(SupervisorExternal = _default_handler);
PROVIDE(MachineExternal = _default_handler);
PROVIDE(DefaultHandler = _default_handler);
PROVIDE(ExceptionHandler = _default_handler);
*/

REGION_ALIAS("REGION_TEXT", ROM);
REGION_ALIAS("REGION_RODATA", ROM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_STACK", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
