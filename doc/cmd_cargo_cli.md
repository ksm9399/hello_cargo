- cargo new {project_name} : 새 프로젝트를 생성할 수 있습니다.

- cargo build : 명령으로 프로젝트를 빌드할 수 있습니다.

- cargo run : 명령어는 한 번에 프로젝트를 빌드하고 실행할 수 있습니다.

- cargo check : 명령으로 바이너리를 생성하지 않고 프로젝트의 에러를 체크할 수 있습니다.

- 빌드로 만들어진 파일은 작성한 소스 코드와 뒤섞이지 않도록 target/debug 디렉터리에 저장됩니다.

***
- cargo build --release : 릴리즈 빌드를 생성
```
일반 빌드와 차이점은 target/debug 가 아닌 target/release 에 실행 파일이 생성

컴파일 시 최적화를 진행하여 컴파일이 오래 걸리는 대신 러스트 코드가 더 빠르게 작동하는 점
```
***

- cargo add {dependencies_name} : 디펜던시(의존성) 추가

- cargo doc --open : 의존하는 크레이트의 문서를 로컬에서 모두 빌드한 다음, 브라우저에서 열어주는 기능