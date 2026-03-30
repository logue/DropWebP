# Docker 빌드 성능 분석

이 문서는 다양한 플랫폼에서 Docker 빌드의 성능 특성과 크로스 컴파일 빌드가 느린 이유를 설명합니다.

## 성능 비교

### 실제 빌드 시간

| 호스트 환경 | 타겟 | 빌드 시간 | QEMU 에뮬레이션 |
|------------|------|----------|----------------|
| macOS (x64) | x64 Linux | 8-12분 | 불필요 ✅ |
| macOS (Apple Silicon) | x64 Linux | 10-15분 | Rosetta 2 사용 |
| Windows (x64) | x64 Linux | 10-15분 | 불필요 ✅ |
| Windows (x64) | ARM64 Linux | **30-60분** | 필요 ❌ |

### 주요 관찰 사항

**왜 Windows x64 → ARM64 Linux 빌드가 이렇게 느린가?**

macOS x64 → x64 Linux 빌드는 빠른 반면, Windows x64 → ARM64 Linux 빌드는 주로 **아키텍처 차이**로 인해 극도로 느립니다.

## 기술적 이유

### 1. QEMU 에뮬레이션 오버헤드

#### 동일한 아키텍처 (빠름)

```bash
# macOS x64 → x64 Linux
호스트: x86_64
타겟: x86_64-unknown-linux-gnu
처리: 네이티브 명령어 실행
속도: 거의 네이티브 속도
```

#### 크로스 아키텍처 (느림)

```bash
# Windows x64 → ARM64 Linux
호스트: x86_64
타겟: aarch64-unknown-linux-gnu
처리: QEMU를 통한 ARM 명령어 에뮬레이션
속도: 10-50배 느림 ❌
```

### 2. 파일 시스템 오버헤드

#### Windows의 I/O 경로

```
C:\Users\...\DropWebP (NTFS)
  ↓ 9P 네트워크 프로토콜
/mnt/c/Users/.../DropWebP (WSL 2)
  ↓ Docker bind mount
/workspace (Container)
```

#### macOS의 I/O 경로

```
/Users/.../DropWebP (APFS)
  ↓ VirtioFS (최적화됨)
/workspace (Container)
```

**VirtioFS**는 가상화 환경을 위해 특별히 설계된 고속 파일 공유 프로토콜입니다:

- 9P보다 2-5배 빠름
- 효율적인 메타데이터 캐싱
- 대규모 프로젝트에 특히 유리

## 성능 최적화 전략

### 권장 접근 방식

#### 개발 단계

```powershell
# x64 Linux만 빌드 (10-15분)
pnpm run build:tauri:linux-x64
```

#### 릴리스 단계

```yaml
# GitHub Actions에서 병렬 빌드
jobs:
  build-x64:
    runs-on: ubuntu-latest
  build-arm64:
    runs-on: ubuntu-latest
```

## 요약

### 핵심 포인트

✅ **동일한 아키텍처는 빠름**: x64 → x64는 10-15분 소요
❌ **크로스 아키텍처는 느림**: x64 → ARM64는 30-60분 소요
🚀 **실용적인 솔루션**: 개발 중에는 x64만, 릴리스는 GitHub Actions

### 개발자를 위한 권장 사항

```bash
# 일상 작업에는 이것으로 충분합니다
pnpm run build:tauri:linux-x64

# 릴리스는 자동화
git tag v3.2.1 && git push origin v3.2.1
```

시간과 리소스를 효율적으로 사용하여 스트레스 없는 개발을 하세요!
