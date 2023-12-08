#!/usr/bin/python3

import argparse
import os
import shutil
import subprocess

parser = argparse.ArgumentParser(
  prog='NewDayCreator',
  description='Create new day for advent of code')
parser.add_argument('day_number', type=int,
                    help='The number of the day to create.')
parser.add_argument('problem_name', type=str,
                    default=None, nargs='?',
                    help='How to name code files.')
parser.add_argument('-ct', '--code_template', type=str,
                    default='rust_template.rs',
                    help='File with code template to copy over.')
parser.add_argument('-mt', '--makefile_template', type=str,
                    default='Makefile_rust_template',
                    help='File with Makefile template to copy over.')
parser.add_argument('--cargo', action='store_true',
                    default=False,
                    help='If use cargo instead of Makefiles')
parser.add_argument('--lib', action='store_true',
                    default=False,
                    help='When true, also copies over the library of codes.')
parser.add_argument('--lib_file', type=str,
                    default='biblioteczka.rs',
                    help='File to use for the library of codes')

CARGO_MAIN_TEMPLATE = """mod {problem_name};
{code_lib}

fn main() {{
    {problem_name}::main();
}}
"""


def makefile_based(day_name: str, problem_name: str, code_template: str,
                   makefile_template: str):
  os.mkdir(day_name)
  code_path = os.path.join(day_name, f'{problem_name}-curr.rs')
  shutil.copyfile(code_template, code_path)
  makefile_path = os.path.join(day_name, 'Makefile')
  shutil.copyfile(makefile_template, makefile_path)


def cargo_based(day_name: str, problem_name: str, code_template: str,
                lib: bool, lib_file: str):
  subprocess.run(['cargo', 'new', day_name, '--bin'], check=True)
  src_path = os.path.join(day_name, 'src')
  main_path = os.path.join(src_path, 'main.rs')
  problem_path = os.path.join(src_path, f'{problem_name}.rs')
  lib_import = f'mod {lib_file[:-len(".rs")]};' if lib else ''

  shutil.copyfile(code_template, problem_path)
  with open(main_path, 'w') as mf:
    mf.write(CARGO_MAIN_TEMPLATE.format(
      problem_name = problem_name, code_lib = lib_import))

  if lib:
    lib_path = os.path.join(src_path, lib_file)
    shutil.copyfile(lib_file, lib_path)


def main():
  args = parser.parse_args()

  day_name = f'day{args.day_number}'
  problem_name = (args.problem_name
                  if args.problem_name is not None
                  else day_name)
  if os.path.isdir(day_name):
    print(f'Directory "{day_name}" already exists, aborting...')
    return

  if not args.cargo:
    makefile_based(day_name, problem_name, args.code_template,
                   args.makefile_template)
  else:
    cargo_based(day_name, problem_name, args.code_template, args.lib,
                args.lib_file)


if __name__ == "__main__":
  main()
