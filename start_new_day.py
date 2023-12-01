#!/usr/bin/python3

import argparse
import os
import shutil

parser = argparse.ArgumentParser(
  prog='NewDayCreator',
  description='Create new day for advent of code')
parser.add_argument('-d', '--day_number', type=int,
                    required=True,
                    help='The number of the day to create.')
parser.add_argument('-ct', '--code_template', type=str,
                    default='rust_template.rs',
                    help='File with code template to copy over.')
parser.add_argument('-mt', '--makefile_template', type=str,
                    default='Makefile_rust_template',
                    help='File with Makefile template to copy over.')

def main():
  args = parser.parse_args()

  day_name = f'day{args.day_number}'
  if os.path.isdir(day_name):
    print(f'Directory "{day_name}" already exists, stopping...')
    return

  os.mkdir(day_name)
  code_path = os.path.join(day_name, f'{day_name}.rs')
  shutil.copyfile(args.code_template, code_path)
  makefile_path = os.path.join(day_name, 'Makefile')
  shutil.copyfile(args.makefile_template, makefile_path)



if __name__ == "__main__":
  main()
