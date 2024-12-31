#!/usr/bin/python3

import argparse
import json
import os
import requests
import shutil
import subprocess

from dataclasses import dataclass
from pathlib import Path

parser = argparse.ArgumentParser(
  prog='ProblemInitializer',
  description='Pulls input and preprares a directory for a new problem.')
parser.add_argument('year', type=int,
                    help='The year to use.')
parser.add_argument('day_number', type=int,
                    help='The number of the day to create.')
parser.add_argument('-td', '--templates_dir', type=str,
                    default='templates',
                    help='Directory with templates to copy over.')
parser.add_argument('--secrets', type=str,
                    default='secrets.json',
                    help='JSON file with secrets, eg. for login')
parser.add_argument('--only_input', action='store_true',
                    default=False,
                    help='Only downloads input for the given day, assumes '
                         'that the day already exists.')


class DownloadManager:
  USER_AGENT = 'https://github.com/anula/old_advent_of_code - auto-get input'

  @staticmethod
  def from_file(secrets_file: Path) -> 'DownloadManager':
    with open(secrets_file) as f:
      secrets = json.load(f)
    return DownloadManager(secrets['aoc_session'])

  def __init__(self, session_token: str):
    self.session_token = session_token

  def get_page(self, url: str) -> bytes:
    failed = False
    try:
      resp = requests.get(
        url,
        cookies={'session': self.session_token},
        headers={
          'User-Agent': DownloadManager.USER_AGENT
        })
    except requests.ConnectionError as err:
      failed = True
      reason = err

    if not resp.ok:
      failed = True
      reason = resp.reason

    if failed:
      raise RuntimeError(f'Failed to download input, reason: {reason}')

    return resp.content


@dataclass
class AOCProblem:
  INPUT_URL = 'https://adventofcode.com/{year}/day/{day_number}/input'

  year: int
  day: int

  def get_input(self, download_manager: DownloadManager) -> bytes:
    url = AOCProblem.INPUT_URL.format(year=self.year, day_number=self.day)
    return download_manager.get_page(url)

  def save_input(self, dest_path: Path, download_manager: DownloadManager):
    with open(dest_path, 'wb') as f:
      f.write(self.get_input(download_manager))


class ProblemDirectory:
  MAIN_FILE = 'main.rs'
  SOLUTION_FILE = 'solution.rs'
  DEPS_FILE = 'dependencies.toml'
  CARGO_TOML_FILE = 'Cargo.toml'

  def __init__(self, year: int, day: int, templates_dir: Path):
    self.year = year
    self.day = day
    self.templates_dir = templates_dir
    self.dir = Path(os.path.join('.', str(self.year), f'day{self.day}'))

  def exists(self) -> bool:
    return self.dir.exists() and self.dir.is_dir()

  def cargo_init(self):
    subprocess.run(['cargo', 'new', self.dir, '--bin', '--vcs=none'], check=True)

    base_path = self.dir

    src_path = base_path / 'src'

    main_template_path = self.templates_dir / ProblemDirectory.MAIN_FILE
    main_path = src_path / ProblemDirectory.MAIN_FILE

    solution_template_path = self.templates_dir / ProblemDirectory.SOLUTION_FILE
    solution_path = src_path / ProblemDirectory.SOLUTION_FILE

    shutil.copyfile(main_template_path, main_path)
    shutil.copyfile(solution_template_path, solution_path)

    deps_path = self.templates_dir / ProblemDirectory.DEPS_FILE
    cargo_toml_path = base_path / ProblemDirectory.CARGO_TOML_FILE

    with open(deps_path, 'r') as df:
      deps_content = df.read()

    with open(cargo_toml_path, 'a') as mf:
      mf.write(deps_content)

  def get_input(self, download_manager):
    aoc = AOCProblem(year=self.year, day=self.day)
    input_path = self.dir / 'input'
    aoc.save_input(input_path, download_manager)


def cargo_init(day_name: str, templates_dir: str):
  subprocess.run(['cargo', 'new', day_name, '--bin'], check=True)
  src_path = os.path.join(day_name, 'src')

  main_template_path = os.path.join(templates_dir, MAIN_FILE)
  main_path = os.path.join(src_path, MAIN_FILE)

  solution_template_path = os.path.join(templates_dir, SOLUTION_FILE)
  solution_path = os.path.join(src_path, SOLUTION_FILE)

  shutil.copyfile(main_template_path, main_path)
  shutil.copyfile(solution_template_path, solution_path)

  deps_path = os.path.join(templates_dir, DEPS_FILE)
  cargo_toml_path = os.path.join(day_name, CARGO_TOML_FILE)

  with open(deps_path, 'r') as df:
    deps_content = df.read()

  with open(cargo_toml_path, 'a') as mf:
    mf.write(deps_content)


def parse_secrets(secrets_file: str):
  with open(secrets_file) as f:
    return json.load(f)


def download_input(
    website_tmpl: str, session_token: str, day_num: int, dest_dir: str):
  dest_path = os.path.join(dest_dir, 'input')

  failed = False
  try:
    resp = requests.get(
      website_tmpl.format(year=YEAR, day_number=day_num),
      cookies={'session': session_token},
      headers={
        'User-Agent':
        f'https://github.com/anula/advent_of_code_{YEAR} - auto-get input'})
  except requests.ConnectionError as err:
    failed = True
    reason = err

  if not resp.ok:
    failed = True
    reason = resp.reason

  if failed:
    print(f'Failed to download input, reason: {reason}')

  with open(dest_path, 'wb') as f:
    f.write(resp.content)


def main():
  args = parser.parse_args()

  problem = ProblemDirectory(year=args.year, day=args.day_number,
                             templates_dir=Path(args.templates_dir))

  if not args.only_input:
    if problem.exists():
      print(f'Directory "{problem.dir}" already exists, aborting...')
      return

    problem.cargo_init()

  if not problem.exists():
    print(f'Directory "{problem.dir}" was not created. Aborting input download...')

  download_manager = DownloadManager.from_file(args.secrets)

  problem.get_input(download_manager)


if __name__ == "__main__":
  main()
