import pygame
import random
import time
import sys
from pygame.locals import *

# 初始化pygame
pygame.init()

# 颜色定义
WHITE = (255, 255, 255)
BLACK = (0, 0, 0)
GRAY = (200, 200, 200)
LIGHT_BLUE = (173, 216, 230)
RED = (255, 0, 0)
GREEN = (0, 255, 0)
DARK_BLUE = (0, 0, 139)

# 游戏设置
WINDOW_SIZE = 540
GRID_SIZE = 9
CELL_SIZE = WINDOW_SIZE // GRID_SIZE
FONT_SIZE = 40
SMALL_FONT_SIZE = 20

# 创建窗口
screen = pygame.display.set_mode((WINDOW_SIZE, WINDOW_SIZE + 120))
pygame.display.set_caption('数独游戏')
font = pygame.font.SysFont('SimSun', FONT_SIZE)
small_font = pygame.font.SysFont('SimSun', SMALL_FONT_SIZE)  # 使用默认字体，这里可以尝试更换为支持中文的字体路径，例如 'C:/Windows/Fonts/simsun.ttc'

class SudokuGenerator:
    def __init__(self):
        self.grid = [[0 for _ in range(GRID_SIZE)] for _ in range(GRID_SIZE)]
    
    def is_valid(self, grid, row, col, num):
        # 检查行
        for x in range(GRID_SIZE):
            if grid[row][x] == num:
                return False
        
        # 检查列
        for x in range(GRID_SIZE):
            if grid[x][col] == num:
                return False
        
        # 检查3x3宫格
        start_row, start_col = 3 * (row // 3), 3 * (col // 3)
        for i in range(3):
            for j in range(3):
                if grid[start_row + i][start_col + j] == num:
                    return False
        
        return True
    
    def solve(self, grid):
        for i in range(GRID_SIZE):
            for j in range(GRID_SIZE):
                if grid[i][j] == 0:
                    for num in range(1, 10):
                        if self.is_valid(grid, i, j, num):
                            grid[i][j] = num
                            if self.solve(grid):
                                return True
                            grid[i][j] = 0
                    return False
        return True
    
    def generate_puzzle(self, difficulty=0.5):
        # 生成完整数独
        self.solve(self.grid)
        
        # 根据难度移除数字
        cells_to_remove = int(GRID_SIZE * GRID_SIZE * difficulty)
        removed_cells = 0
        
        while removed_cells < cells_to_remove:
            row, col = random.randint(0, 8), random.randint(0, 8)
            if self.grid[row][col] != 0:
                self.grid[row][col] = 0
                removed_cells += 1
        
        return self.grid

class SudokuGame:
    def __init__(self):
        self.generator = SudokuGenerator()
        self.board = None
        self.solution = None
        self.user_board = None
        self.selected_cell = None
        self.start_time = None
        self.elapsed_time = 0
        self.game_over = False
        self.difficulty = 0.5
    
    def new_game(self):
        # 生成新游戏
        self.board = self.generator.generate_puzzle(self.difficulty)
        self.user_board = [row[:] for row in self.board]
        
        # 获取完整解
        self.solution = [row[:] for row in self.board]
        temp_generator = SudokuGenerator()
        temp_generator.solve(self.solution)
        
        self.selected_cell = None
        self.start_time = time.time()
        self.elapsed_time = 0
        self.game_over = False
    
    def check_win(self):
        for i in range(GRID_SIZE):
            for j in range(GRID_SIZE):
                if self.user_board[i][j] != self.solution[i][j]:
                    return False
        return True
    
    def update_time(self):
        if not self.game_over and self.start_time:
            self.elapsed_time = time.time() - self.start_time
    
    def get_time_str(self):
        minutes = int(self.elapsed_time) // 60
        seconds = int(self.elapsed_time) % 60
        return f"{minutes:02d}:{seconds:02d}"

def draw_grid():
    # 绘制背景
    screen.fill(WHITE)
    
    # 绘制单元格
    for i in range(GRID_SIZE):
        for j in range(GRID_SIZE):
            rect = pygame.Rect(j * CELL_SIZE, i * CELL_SIZE, CELL_SIZE, CELL_SIZE)
            pygame.draw.rect(screen, WHITE, rect)
            pygame.draw.rect(screen, GRAY, rect, 1)
    
    # 绘制粗线分隔九宫格
    for i in range(0, GRID_SIZE + 1, 3):
        pygame.draw.line(screen, BLACK, (0, i * CELL_SIZE), (WINDOW_SIZE, i * CELL_SIZE), 3)
        pygame.draw.line(screen, BLACK, (i * CELL_SIZE, 0), (i * CELL_SIZE, WINDOW_SIZE), 3)

def draw_numbers(game):
    for i in range(GRID_SIZE):
        for j in range(GRID_SIZE):
            if game.user_board[i][j] != 0:
                # 原始数字（黑色）
                if game.board[i][j] != 0:
                    color = BLACK
                # 用户输入的正确数字（蓝色）
                elif game.user_board[i][j] == game.solution[i][j]:
                    color = DARK_BLUE
                # 用户输入的错误数字（红色）
                else:
                    color = RED
                
                number_text = font.render(str(game.user_board[i][j]), True, color)
                text_rect = number_text.get_rect(center=(j * CELL_SIZE + CELL_SIZE // 2, 
                                                         i * CELL_SIZE + CELL_SIZE // 2))
                screen.blit(number_text, text_rect)

def draw_selected_cell(game):
    if game.selected_cell:
        row, col = game.selected_cell
        rect = pygame.Rect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)
        pygame.draw.rect(screen, LIGHT_BLUE, rect, 3)

def draw_ui(game):
    # 绘制时间
    time_text = small_font.render(f"耗时: {game.get_time_str()}", True, BLACK)
    screen.blit(time_text, (10, WINDOW_SIZE + 10))
    
    # 绘制按钮
    new_game_btn = pygame.Rect(WINDOW_SIZE - 120, WINDOW_SIZE + 10, 100, 30)
    pygame.draw.rect(screen, GREEN, new_game_btn)
    new_game_text = small_font.render("新游戏", True, BLACK)
    screen.blit(new_game_text, (WINDOW_SIZE - 110, WINDOW_SIZE + 15))
    
    # 绘制难度选择
    diff_text = small_font.render("难度:", True, BLACK)
    screen.blit(diff_text, (10, WINDOW_SIZE + 40))
    
    diff_buttons = [
        ("简单", 0.3, 80),
        ("中等", 0.5, 140),
        ("困难", 0.7, 200)
    ]
    
    for text, diff, x_pos in diff_buttons:
        btn = pygame.Rect(x_pos, WINDOW_SIZE + 40, 50, 20)
        color = DARK_BLUE if game.difficulty == diff else GRAY
        pygame.draw.rect(screen, color, btn)
        diff_text = small_font.render(text, True, WHITE)
        screen.blit(diff_text, (x_pos + 5, WINDOW_SIZE + 42))
    
    return new_game_btn, diff_buttons

def show_message(message):
    overlay = pygame.Surface((WINDOW_SIZE, WINDOW_SIZE))
    overlay.set_alpha(200)
    overlay.fill(WHITE)
    screen.blit(overlay, (0, 0))
    
    msg_text = small_font.render(message, True, GREEN)
    text_rect = msg_text.get_rect(center=(WINDOW_SIZE // 2, WINDOW_SIZE // 2))
    screen.blit(msg_text, text_rect)
    
    pygame.display.flip()
    pygame.time.wait(2000)

def main():
    game = SudokuGame()
    game.new_game()
    clock = pygame.time.Clock()
    running = True
    
    while running:
        for event in pygame.event.get():
            if event.type == QUIT:
                running = False
            
            elif event.type == MOUSEBUTTONDOWN:
                x, y = pygame.mouse.get_pos()
                
                # 检查是否点击了网格
                if y < WINDOW_SIZE:
                    col = x // CELL_SIZE
                    row = y // CELL_SIZE
                    game.selected_cell = (row, col)
                
                # 检查按钮点击
                new_game_btn, diff_buttons = draw_ui(game)
                if new_game_btn.collidepoint(x, y):
                    game.new_game()
                
                # 检查难度选择
                for text, diff, x_pos in diff_buttons:
                    btn = pygame.Rect(x_pos, WINDOW_SIZE + 40, 50, 20)
                    if btn.collidepoint(x, y):
                        game.difficulty = diff
                        game.new_game()
            
            elif event.type == KEYDOWN:
                if game.selected_cell and not game.game_over:
                    row, col = game.selected_cell
                    
                    # 只能修改空白格子
                    if game.board[row][col] == 0:
                        if event.key in (K_1, K_KP1):
                            game.user_board[row][col] = 1
                        elif event.key in (K_2, K_KP2):
                            game.user_board[row][col] = 2
                        elif event.key in (K_3, K_KP3):
                            game.user_board[row][col] = 3
                        elif event.key in (K_4, K_KP4):
                            game.user_board[row][col] = 4
                        elif event.key in (K_5, K_KP5):
                            game.user_board[row][col] = 5
                        elif event.key in (K_6, K_KP6):
                            game.user_board[row][col] = 6
                        elif event.key in (K_7, K_KP7):
                            game.user_board[row][col] = 7
                        elif event.key in (K_8, K_KP8):
                            game.user_board[row][col] = 8
                        elif event.key in (K_9, K_KP9):
                            game.user_board[row][col] = 9
                        elif event.key == K_BACKSPACE or event.key == K_DELETE or event.key == K_0:
                            game.user_board[row][col] = 0
                        
                        # 检查是否获胜
                        if game.check_win():
                            game.game_over = True
                            show_message("恭喜！你赢了！")
                            game.new_game()
        
        # 更新时间
        game.update_time()
        
        # 绘制游戏
        draw_grid()
        draw_numbers(game)
        draw_selected_cell(game)
        draw_ui(game)
        
        pygame.display.flip()
        clock.tick(30)
    
    pygame.quit()
    sys.exit()

if __name__ == "__main__":
    main()